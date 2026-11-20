//! `Debug` for [`Tree`]: a dump to read, not to parse.
//!
//! `{:?}` is BOUNDED — a summary line, then every root with its subtree to [`DEPTH`] levels and
//! [`LINES`] node lines in all, what it leaves out counted ("… 9,870 more nodes") — so a tree of ten
//! thousand nodes prints a screenful. `{:#?}` prints every node.
//!
//! A node's line: its id (`n12`, the slot), its published box (`@ x,y w×h`, or `not laid out`), its
//! flags (`measured`, `hug`, `dirty`), and the fields of its style that differ from the default, in
//! CSS's names. A root says what makes it live (its viewport, its host) and its scope host.

use std::fmt::{self, Write as _};

use crate::{Edges, Length, NodeId, Style, Tree, Viewport};

/// How deep the bounded dump goes under each root.
const DEPTH: usize = 3;
/// How many node lines the bounded dump prints in all.
const LINES: usize = 64;

impl fmt::Debug for Tree {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let all = f.alternate();
        let alive: Vec<NodeId> = (0..self.meta.len())
            .filter(|&slot| self.meta[slot].alive)
            .map(|slot| NodeId::new(slot, self.meta[slot].generation))
            .collect();
        let roots: Vec<NodeId> = alive.iter().copied().filter(|&node| self.links[node.slot()].parent.is_none()).collect();
        writeln!(
            f,
            "Tree {{ {} nodes, {} roots ({} live), scale {}, needs_layout: {} }}",
            alive.len(),
            roots.len(),
            self.roots.len(),
            self.scale,
            self.needs_layout()
        )?;
        let mut budget = if all { usize::MAX } else { LINES };
        for root in roots {
            // Preorder, by an explicit stack (a deep tree would overflow a recursion's).
            let mut left_out = 0usize;
            let mut stack = vec![(root, 0usize)];
            while let Some((node, depth)) = stack.pop() {
                if budget > 0 && (all || depth < DEPTH) {
                    budget -= 1;
                    writeln!(f, "{:indent$}{}", "", self.line(node), indent = depth * 2)?;
                } else {
                    left_out += 1;
                }
                let first = stack.len();
                stack.extend(self.children(node).map(|child| (child, depth + 1)));
                stack[first..].reverse();
            }
            if left_out > 0 {
                writeln!(f, "  … {left_out} more nodes under n{} (`{{:#?}}` prints them)", root.index())?;
            }
        }
        Ok(())
    }
}

impl Tree {

    /// One node: id, box, flags, root links, the style's differences from the default.
    fn line(&self, node: NodeId) -> String {
        let slot = node.slot();
        let meta = &self.meta[slot];
        let mut out = format!("n{}", node.index());
        if meta.published {
            let rect = self.out[slot].rect;
            let _ = write!(out, " @ {},{} {}×{}", rect.x, rect.y, rect.width, rect.height);
        } else {
            out.push_str(" not laid out");
        }
        for (on, name) in [(meta.measured, "measured"), (meta.hug, "hug"), (meta.dirty, "dirty")] {
            if on {
                let _ = write!(out, " {name}");
            }
        }
        match meta.viewport {
            Some(Viewport::Size(size)) => {
                let axis = |value: Option<f32>| value.map_or("any".to_string(), |value| value.to_string());
                let _ = write!(out, " [viewport {}×{}]", axis(size.width), axis(size.height));
            }
            Some(Viewport::Host(host)) => {
                let _ = write!(out, " [hosted in n{}]", host.index());
            }
            None => {}
        }
        if let Some(host) = meta.scope_host {
            let _ = write!(out, " [scope host n{}]", host.index());
        }
        let style = style_diff(&self.styles[slot].authored);
        if !style.is_empty() {
            let _ = write!(out, " {{ {style} }}");
        }
        out
    }
}

fn length(value: &Length) -> String {
    match value {
        Length::Auto => "auto".to_string(),
        Length::Px(px) => format!("{px}px"),
        Length::Percent(percent) => format!("{percent}%"),
        Length::Var(var, factor) => format!("{factor}·var{}", var.index()),
        Length::Calc(_) => "calc(…)".to_string(),
    }
}

fn edges(value: &Edges<Length>) -> String {
    let Edges { left, top, right, bottom } = value;
    if left == top && top == right && right == bottom {
        length(left)
    } else {
        format!("{} {} {} {}", length(top), length(right), length(bottom), length(left))
    }
}

/// The style's fields that differ from `Style::default()`, as `name: value` in CSS's names.
fn style_diff(style: &Style) -> String {
    let base = Style::default();
    let mut out: Vec<String> = Vec::new();
    let mut field = |differs: bool, name: &str, value: String| {
        if differs {
            out.push(format!("{name}: {value}"));
        }
    };
    field(style.display != base.display, "display", format!("{:?}", style.display));
    field(style.position != base.position, "position", format!("{:?}", style.position));
    field(style.box_sizing != base.box_sizing, "box-sizing", format!("{:?}", style.box_sizing));
    field(style.overflow != base.overflow, "overflow", format!("{:?}", style.overflow));
    field(style.flex_direction != base.flex_direction, "flex-direction", format!("{:?}", style.flex_direction));
    field(style.flex_wrap != base.flex_wrap, "flex-wrap", format!("{:?}", style.flex_wrap));
    field(style.justify_content != base.justify_content, "justify-content", format!("{:?}", style.justify_content));
    field(style.align_items != base.align_items, "align-items", format!("{:?}", style.align_items));
    field(style.align_self != base.align_self, "align-self", format!("{:?}", style.align_self));
    field(style.align_content != base.align_content, "align-content", format!("{:?}", style.align_content));
    field(style.flex_grow != base.flex_grow, "flex-grow", style.flex_grow.to_string());
    field(style.flex_shrink != base.flex_shrink, "flex-shrink", style.flex_shrink.to_string());
    field(style.flex_basis != base.flex_basis, "flex-basis", length(&style.flex_basis));
    field(style.size.width != base.size.width, "width", length(&style.size.width));
    field(style.size.height != base.size.height, "height", length(&style.size.height));
    field(style.min_size.width != base.min_size.width, "min-width", length(&style.min_size.width));
    field(style.min_size.height != base.min_size.height, "min-height", length(&style.min_size.height));
    field(style.max_size.width != base.max_size.width, "max-width", length(&style.max_size.width));
    field(style.max_size.height != base.max_size.height, "max-height", length(&style.max_size.height));
    field(style.aspect_ratio != base.aspect_ratio, "aspect-ratio", format!("{:?}", style.aspect_ratio));
    field(style.margin != base.margin, "margin", edges(&style.margin));
    field(style.padding != base.padding, "padding", edges(&style.padding));
    field(style.border != base.border, "border", edges(&style.border));
    field(style.inset != base.inset, "inset", edges(&style.inset));
    field(style.row_gap != base.row_gap, "row-gap", length(&style.row_gap));
    field(style.column_gap != base.column_gap, "column-gap", length(&style.column_gap));
    out.join(", ")
}
