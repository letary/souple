//! The engine: the per-node storage, the node lifecycle, and the pass.
//!
//! Storage is one array per ROLE, all indexed by a node's slot: `links` (the tree), `styles`,
//! `meta` (flags and variables), `raw` (what the algorithm wrote), `out` (what the embedder reads),
//! `cache` (the algorithm's). The tree edits are in `edits.rs`, the variables in `vars.rs`, the
//! flexbox algorithm in `algo/`.

use std::num::NonZeroU32;

use crate::algo;
use crate::edits::Links;
use crate::vars::Pending;
use crate::{Edges, Measurer, NodeId, Rect, Size, Style, VarId, VarSet, MAX_VARS};

/// What makes a root LIVE: the box it is laid out in. A root without one (a subtree under
/// construction, a screen that is not shown) is never laid out.
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Viewport {
    /// A box of the embedder's: a screen in its window. `None` on an axis = unbounded (a list item:
    /// a known width, any height); so is an axis that is not a finite number.
    Size(Size<Option<f32>>),
    /// The content box of a node of ANOTHER tree: a pager's page in its pager, a dialog in its
    /// screen. The root is laid out after its host and follows its size; its variables continue up
    /// the host's chain. Its box is relative to the host's content box.
    Node(NodeId),
}

impl Viewport {
    /// A box of this size.
    pub fn size(width: f32, height: f32) -> Self {
        Self::Size(Size::new(Some(width), Some(height)))
    }

    /// This wide, as tall as the content: a list item.
    pub fn width(width: f32) -> Self {
        Self::Size(Size::new(Some(width), None))
    }

    /// This tall, as wide as the content.
    pub fn height(height: f32) -> Self {
        Self::Size(Size::new(None, Some(height)))
    }

    /// No bound on either axis: the root is the size of its content.
    pub fn unbounded() -> Self {
        Self::Size(Size::new(None, None))
    }

    /// The content box of `host`, a node of another tree.
    pub fn node(host: NodeId) -> Self {
        Self::Node(host)
    }

    /// An axis that is not a finite number is unbounded.
    fn sanitized(self) -> Self {
        match self {
            Self::Size(size) => {
                let axis = |value: Option<f32>| value.filter(|value| value.is_finite());
                Self::Size(Size::new(axis(size.width), axis(size.height)))
            }
            node => node,
        }
    }
}

/// The laid-out box of a node, in logical px, snapped to the device grid. Every number is finite.
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct NodeLayout {
    /// The border box; the origin is relative to the parent's border box (a hosted root's: to its
    /// host's content box).
    pub rect: Rect,
    /// The resolved margins, unrounded; an `auto` margin is 0 here (the space it took is in the
    /// origin). What an embedder sums its own extents with (a scroll view's content height).
    pub margin: Edges<f32>,
    /// The resolved padding, unrounded.
    pub padding: Edges<f32>,
    /// The resolved border widths, unrounded.
    pub border: Edges<f32>,
    /// The extent of the content, from the padding box's origin: what a scroll container scrolls.
    pub content_size: Size<f32>,
}

/// A number as published: a NaN is 0, an infinity the largest finite `f32` of its sign, any other
/// number itself. Only a degenerate input (a NaN, an overflow) ever needs it.
pub(crate) fn finite(value: f32) -> f32 {
    if value.is_nan() { 0.0 } else { value.clamp(f32::MIN, f32::MAX) }
}

/// The nearest whole number, a tie toward +∞ (`floor(x + 0.5)` in exact arithmetic): the rule that
/// commutes with a move by a whole number, for either sign — −0.5 goes to 0 as 0.5 goes to 1 — so a
/// subtree moved by whole device pixels keeps the edges a pass from scratch gives it. `f32::round`
/// (a tie away from zero) sends −0.5 to −1. Written without the `+ 0.5`, which rounds in f32
/// (0.49999997 + 0.5 is 1.0; an odd number past 2^23 plus 0.5 is the next even one): `x - floor(x)`
/// is exact wherever it is near the tie. A NaN stays NaN, an infinity itself.
fn round_half_up(x: f32) -> f32 {
    let down = x.floor();
    if x - down >= 0.5 { down + 1.0 } else { down }
}

/// [`finite`] on each edge.
pub(crate) fn finite_edges(edges: Edges<f32>) -> Edges<f32> {
    let Edges { left, top, right, bottom } = edges;
    // The common case first: a sum is finite only if every term is (an overflow of the sum itself
    // costs the slow path, nothing more).
    if (left + top + right + bottom).is_finite() {
        return edges;
    }
    Edges { left: finite(left), top: finite(top), right: finite(right), bottom: finite(bottom) }
}

impl NodeLayout {
    /// Every number finite: the promise of the published layout. The margins, the padding and the
    /// border are made so where the algorithm writes them (`algo::finish`), once per layout; the
    /// rect and the content size here, once per publish. An ordinary box passes through bit for bit.
    fn finite(self) -> Self {
        let Self { rect, content_size, .. } = self;
        if (rect.x + rect.y + rect.width + rect.height + content_size.width + content_size.height).is_finite() {
            return self;
        }
        Self {
            rect: Rect { x: finite(rect.x), y: finite(rect.y), width: finite(rect.width), height: finite(rect.height) },
            content_size: Size::new(finite(content_size.width), finite(content_size.height)),
            ..self
        }
    }
}

/// A node whose published layout differs from the last pass's — or that is published for the
/// first time: the embedder has never seen its box, even one equal to the default (a 0 × 0 box at
/// the origin). The flags compare with the last box, the default one for a first publish (so all
/// three are false for an empty box at the origin).
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Change {
    /// The node; [`Tree::layout`] has its new box.
    pub node: NodeId,
    /// The origin, relative to the parent.
    pub moved: bool,
    /// The width or the height.
    pub resized: bool,
    /// The margin, the padding, the border or the content size.
    pub inner: bool,
}

pub(crate) struct Styles {
    /// What the embedder wrote.
    pub authored: Style,
    /// The same with the variables in scope substituted — kept only for a style that reads a
    /// variable. Most do not, and their computed style IS the authored one.
    pub resolved: Option<Box<Style>>,
}

impl Styles {
    fn new() -> Self {
        Self { authored: Style::default(), resolved: None }
    }

    /// What the algorithm reads: px and percent only.
    pub fn computed(&self) -> &Style {
        self.resolved.as_deref().unwrap_or(&self.authored)
    }
}

pub(crate) struct Meta {
    pub generation: NonZeroU32,
    pub alive: bool,
    /// A content-sized leaf: sized by the [`Measurer`].
    pub measured: bool,
    /// A measured leaf that HUGS its content (`Tree::set_hug`).
    pub hug: bool,
    /// Something under this node changed since its root was last laid out.
    pub dirty: bool,
    /// The edit epoch (`Tree::pass + 1`) of the last `mark_dirty` that went through this node.
    pub dirtied: u32,
    /// The resolve sweep (`Tree::sweep`) that last computed this node's style, and the one that
    /// last finished everything in its scope: a second resolve in the same sweep has nothing to do.
    pub resolved_in: u32,
    pub scope_resolved_in: u32,
    /// The algorithm ran this node's own final layout in this pass — it placed the children again.
    /// Set by `algo`, cleared by the publish walk.
    pub laid: bool,
    /// The variables the node's own style reads.
    pub uses: VarSet,
    /// The variables read anywhere at or under the node (hosted roots included). Only ever grows:
    /// it prunes walks, so too large costs a walk and nothing else.
    pub subtree_uses: VarSet,
    /// The variables the node defines.
    pub defs: Vec<(VarId, f32)>,
    /// A root is the viewport of some other root (`Viewport::Node`), or its scope host
    /// (`Tree::set_scope_host`). Only ever set.
    pub hosts: bool,
    /// A root's scope host (`Tree::set_scope_host`): where its scope chain goes on.
    pub scope_host: Option<NodeId>,
    pub viewport: Option<Viewport>,
    /// The size the root was last laid out in.
    pub laid_in: Option<Size<Option<f32>>>,
    /// The pass that last visited this root.
    pub pass: u32,
    /// Where the node's origin sat between two device pixels at the last publish (NaN past 2^24
    /// device px). The snapped boxes of a subtree depend on its root's position only through this
    /// (in exact arithmetic: `Tree::publish`).
    pub phase: (f32, f32),
    /// The scale of the node's last publish (0 before the first): a node published at another
    /// scale is snapped again with its whole subtree, wherever it is published next — under a root
    /// that had no viewport when the scale changed, or moved into another tree since.
    pub scale: f32,
    /// A pass has published the node's layout: its first publish is a change whatever the box.
    pub published: bool,
}

/// A forest of nodes, their styles, and the published boxes: the engine.
///
/// The embedder builds and edits trees ([`Tree::create_node`], [`Tree::insert`],
/// [`Tree::set_style`]), gives the roots it shows a [`Viewport`], and calls
/// [`Tree::compute_layout`] once per frame; the pass lays out what changed, and [`Tree::changes`]
/// then names the nodes whose box moved. A node is a flex container and a flex item at once; a node
/// marked measured ([`Tree::set_measured`]) is sized by the [`Measurer`].
///
/// A `NodeId` that is stale (its node was destroyed) panics in every method that takes one; so does
/// an edit that the method documents as refused (a cycle, a viewport on a child). Numbers never
/// panic.
pub struct Tree {
    pub(crate) links: Vec<Links>,
    pub(crate) styles: Vec<Styles>,
    pub(crate) meta: Vec<Meta>,
    pub(crate) raw: Vec<NodeLayout>,
    pub(crate) out: Vec<NodeLayout>,
    pub(crate) cache: Vec<algo::Cache>,
    /// The algorithm's working memory, reused from pass to pass (one per nesting level in use).
    pub(crate) scratch: Vec<algo::Scratch>,
    free: Vec<usize>,

    /// The live roots, in the order they got a viewport.
    pub(crate) roots: Vec<NodeId>,
    /// (host, root) of every `Viewport::Node`.
    pub(crate) hosted: Vec<(NodeId, NodeId)>,
    /// (host, root) of every scope host (`Tree::set_scope_host`).
    pub(crate) scope_hosted: Vec<(NodeId, NodeId)>,
    pub(crate) pending: Vec<Pending>,
    /// The value of each variable where no node up the chain defines it (`set_global`).
    pub(crate) globals: [f32; MAX_VARS as usize],
    /// The globals set since the last pass: every style that reads one is resolved again.
    pub(crate) globals_changed: VarSet,
    pub(crate) needs_layout: bool,
    changes: Vec<Change>,
    /// The device px per logical px. A box published at another one is snapped again, its whole
    /// subtree with it, the ones the algorithm answered from its cache included (`Meta::scale`).
    scale: f32,
    pass: u32,
    /// Counts the resolve sweeps (`vars.rs`).
    pub(crate) sweep: u32,
}

impl Default for Tree {
    fn default() -> Self {
        Self::new()
    }
}

impl Tree {
    /// An empty tree, at scale 1.
    pub fn new() -> Self {
        Self {
            links: Vec::new(),
            styles: Vec::new(),
            meta: Vec::new(),
            raw: Vec::new(),
            out: Vec::new(),
            cache: Vec::new(),
            scratch: Vec::new(),
            free: Vec::new(),
            roots: Vec::new(),
            hosted: Vec::new(),
            scope_hosted: Vec::new(),
            pending: Vec::new(),
            globals: [0.0; MAX_VARS as usize],
            globals_changed: VarSet::EMPTY,
            needs_layout: false,
            changes: Vec::new(),
            scale: 1.0,
            pass: 0,
            sweep: 0,
        }
    }

    /// A tree with room for `nodes` nodes: building it grows no array.
    pub fn with_capacity(nodes: usize) -> Self {
        let mut tree = Self::new();
        tree.reserve(nodes);
        tree
    }

    /// Makes room for `additional` more nodes.
    pub fn reserve(&mut self, additional: usize) {
        self.links.reserve(additional);
        self.styles.reserve(additional);
        self.meta.reserve(additional);
        self.raw.reserve(additional);
        self.out.reserve(additional);
        self.cache.reserve(additional);
    }

    // ── nodes ────────────────────────────────────────────────────────────────────────────────

    /// A new detached node with the default style.
    pub fn create_node(&mut self) -> NodeId {
        self.push_node(Styles::new())
    }

    /// A new detached node with `style`: what `create_node` and then `set_style` leave, without
    /// the work of REPLACING a style — comparing the new one with the default, dropping the
    /// default, marking the node dirty (a new node is dirty, and its cache empty, already). The
    /// way in for a builder.
    pub fn create_node_with_style(&mut self, style: Style) -> NodeId {
        let uses = style.vars();
        let node = self.push_node(Styles { authored: style, resolved: None });
        if !uses.is_empty() {
            self.meta[node.slot()].uses = uses;
            self.add_uses(node, uses);
            // Resolved at the next pass, in the node's final place (as `set_style` does).
            self.pending.push(Pending { node, vars: uses, subtree: false });
        }
        node
    }

    /// A live node holding `styles`: in a freed slot (`destroy` left it as new), else at the end of
    /// every array.
    fn push_node(&mut self, styles: Styles) -> NodeId {
        if let Some(slot) = self.free.pop() {
            self.meta[slot].alive = true;
            self.styles[slot] = styles;
            return NodeId::new(slot, self.meta[slot].generation);
        }
        let slot = self.meta.len();
        let generation = NonZeroU32::MIN;
        self.links.push(Links::default());
        self.styles.push(styles);
        self.meta.push(Meta::new(generation));
        self.raw.push(NodeLayout::default());
        self.out.push(NodeLayout::default());
        self.cache.push(algo::Cache::default());
        NodeId::new(slot, generation)
    }

    /// Frees the node and its whole subtree; their ids are stale from now on. A root hosted by a
    /// freed node loses its viewport (`Viewport::Node`) or its scope host.
    pub fn destroy(&mut self, node: NodeId) {
        self.slot(node);
        if self.links[node.slot()].parent.is_some() {
            self.detach(node);
        }
        let mut subtree = vec![node];
        let mut next = 0;
        while next < subtree.len() {
            let mut child = self.links[subtree[next].slot()].first_child;
            while let Some(id) = child {
                subtree.push(id);
                child = self.links[id.slot()].next;
            }
            next += 1;
        }
        for id in subtree {
            self.set_viewport_unchecked(id, None);
            self.set_scope_host_unchecked(id, None);
            if self.meta[id.slot()].hosts {
                let orphans: Vec<NodeId> =
                    self.hosted.iter().filter(|(host, _)| *host == id).map(|(_, root)| *root).collect();
                for root in orphans {
                    self.set_viewport_unchecked(root, None);
                }
                let orphans: Vec<NodeId> =
                    self.scope_hosted.iter().filter(|(host, _)| *host == id).map(|(_, root)| *root).collect();
                for root in orphans {
                    self.set_scope_host_unchecked(root, None);
                }
            }
            let slot = id.slot();
            let generation = self.meta[slot].generation.checked_add(1).unwrap_or(NonZeroU32::MIN);
            self.links[slot] = Links::default();
            self.styles[slot] = Styles::new();
            self.meta[slot] = Meta::new(generation);
            self.meta[slot].alive = false;
            self.raw[slot] = NodeLayout::default();
            self.out[slot] = NodeLayout::default();
            self.cache[slot] = algo::Cache::default();
            self.free.push(slot);
        }
    }

    /// The node exists: it was made by this tree and not destroyed since. The one call that takes a
    /// stale id without panicking.
    pub fn is_alive(&self, node: NodeId) -> bool {
        self.meta.get(node.slot()).is_some_and(|meta| meta.alive && meta.generation == node.generation())
    }

    /// The node's slot; panics on a stale id.
    pub(crate) fn slot(&self, node: NodeId) -> usize {
        assert!(self.is_alive(node), "a stale NodeId: the node was destroyed");
        node.slot()
    }

    // ── style ────────────────────────────────────────────────────────────────────────────────

    /// The node's style as written (variables and all, not their values).
    pub fn style(&self, node: NodeId) -> &Style {
        &self.styles[self.slot(node)].authored
    }

    /// Replaces the node's style. The same style again costs a comparison and dirties nothing.
    pub fn set_style(&mut self, node: NodeId, style: Style) {
        let slot = self.slot(node);
        if self.styles[slot].authored == style {
            return;
        }
        let uses = style.vars();
        self.meta[slot].uses = uses;
        self.add_uses(node, uses);
        if uses.is_empty() {
            // No variable: the style is its own computed form. It differs from the authored one
            // (checked above), so only a style that HAD variables needs comparing again.
            let changed = self.styles[slot].resolved.as_deref().is_none_or(|computed| *computed != style);
            self.styles[slot] = Styles { authored: style, resolved: None };
            if changed {
                self.mark_dirty(node);
            }
        } else {
            // Resolved at the next pass: the scope may still change (a node styled, then inserted).
            self.styles[slot].authored = style;
            self.pending.push(Pending { node, vars: uses, subtree: false });
        }
    }

    /// Edits the node's style in place: `tree.update_style(node, |style| style.flex_grow = 1.0)`.
    pub fn update_style(&mut self, node: NodeId, edit: impl FnOnce(&mut Style)) {
        let mut style = self.style(node).clone();
        edit(&mut style);
        self.set_style(node, style);
    }

    /// Makes the node a content-sized leaf (or a plain box again). A measured leaf's children are
    /// not laid out.
    pub fn set_measured(&mut self, node: NodeId, measured: bool) {
        let slot = self.slot(node);
        if self.meta[slot].measured != measured {
            self.meta[slot].measured = measured;
            self.mark_dirty(node);
        }
    }

    /// Makes a measured leaf HUG its content (or follow CSS again, the default): its box is as
    /// wide as its content says it is, as Yoga, React Native, SwiftUI and Flutter size text.
    ///
    /// Where CSS asks a leaf for its max-content width — a row's flex base size, an auto-width
    /// container's max-content contribution — a hugging leaf is instead offered the container's
    /// available width (all of it, not what its siblings leave) and keeps what the Measurer
    /// answers: a text that wraps inside 362 px and reports a longest line of 306 is 306 wide; an
    /// ellipsized line is the width of what it kept. Whatever fixes the width still wins: a
    /// definite `width`, a stretch, a flex-grow share, a shrink, min and max. On a node that is not
    /// measured the flag does nothing.
    pub fn set_hug(&mut self, node: NodeId, hug: bool) {
        let slot = self.slot(node);
        if self.meta[slot].hug != hug {
            self.meta[slot].hug = hug;
            self.mark_dirty(node);
        }
    }

    /// The node's layout is stale: its content changed (a measured leaf's text), so it and its
    /// ancestors are laid out again by the next pass.
    pub fn mark_dirty(&mut self, node: NodeId) {
        self.slot(node);
        // The walk stops at a node it already went through since the last pass: everything above
        // is marked, and no cache was filled since. The `dirty` flag cannot say that — a subtree no
        // pass reaches (display none, a root without a viewport) stays dirty under clean ancestors
        // — so the stop is by the epoch. A node that changes parent keeps the invariant because
        // `insert` marks the new parent itself.
        let epoch = self.pass.wrapping_add(1);
        let mut at = node;
        loop {
            let slot = at.slot();
            if self.meta[slot].dirtied == epoch {
                return;
            }
            self.meta[slot].dirtied = epoch;
            self.meta[slot].dirty = true;
            self.cache[slot].clear();
            match self.links[slot].parent {
                Some(parent) => at = parent,
                None => {
                    if self.meta[slot].viewport.is_some() {
                        self.needs_layout = true;
                    }
                    return;
                }
            }
        }
    }

    // ── roots ────────────────────────────────────────────────────────────────────────────────

    /// Gives a root its viewport: it is laid out by every pass from now on. An axis of a
    /// `Viewport::Size` that is not a finite number is unbounded, as `None` is.
    ///
    /// # Panics
    ///
    /// If `root` has a parent, or `host` (of a `Viewport::Node`) is `root` or leads up to it — by
    /// its parents, or a root's viewport host or scope host, whichever is in use.
    pub fn set_viewport(&mut self, root: NodeId, viewport: Viewport) {
        let slot = self.slot(root);
        assert!(self.links[slot].parent.is_none(), "only a root has a viewport");
        if let Viewport::Node(host) = viewport {
            self.slot(host);
            assert!(!self.leads_to(host, root), "a root cannot be hosted inside itself");
        }
        self.set_viewport_unchecked(root, Some(viewport.sanitized()));
    }

    /// Takes the root's viewport away: no pass lays it out, its boxes stay as they are.
    pub fn clear_viewport(&mut self, root: NodeId) {
        self.slot(root);
        self.set_viewport_unchecked(root, None);
    }

    fn set_viewport_unchecked(&mut self, root: NodeId, viewport: Option<Viewport>) {
        let slot = root.slot();
        let old = self.meta[slot].viewport;
        if old == viewport {
            return;
        }
        if let Some(Viewport::Node(_)) = old {
            self.hosted.retain(|(_, hosted)| *hosted != root);
        }
        match (old.is_some(), viewport.is_some()) {
            (false, true) => self.roots.push(root),
            (true, false) => self.roots.retain(|live| *live != root),
            _ => {}
        }
        self.meta[slot].viewport = viewport;
        if let Some(Viewport::Node(host)) = viewport {
            self.hosted.push((host, root));
            self.meta[host.slot()].hosts = true;
        }
        if viewport.is_some() {
            self.needs_layout = true;
        }
        // The scope chain changed only when the host did: it now ends at the new host, or at
        // nothing. A size given, taken and given again (an embedder that lays a root out on
        // demand) leaves the chain — and every style resolved in it — as it was.
        let host = |viewport: Option<Viewport>| match viewport {
            Some(Viewport::Node(host)) => Some(host),
            _ => None,
        };
        if host(old) != host(viewport) {
            self.rescope_later(root);
        }
    }

    /// `from` is `target`, or one of the links a scope chain may follow leads from it up to
    /// `target`: a parent, else a root's viewport host AND its scope host — the one in use and the
    /// one it shadows, which comes back into use when the viewport goes. A link is made only where
    /// this is false, so no edit, later or earlier, closes a loop.
    pub(crate) fn leads_to(&self, from: NodeId, target: NodeId) -> bool {
        let mut at = from;
        // A root with both links leaves the second for later; one already gone past adds nothing.
        let mut later: Vec<NodeId> = Vec::new();
        let mut forked: Vec<NodeId> = Vec::new();
        loop {
            if at == target {
                return true;
            }
            let slot = at.slot();
            let up = match self.links[slot].parent {
                Some(parent) => Some(parent),
                None => {
                    let viewport_host = match self.meta[slot].viewport {
                        Some(Viewport::Node(host)) => Some(host),
                        _ => None,
                    };
                    match (viewport_host, self.meta[slot].scope_host) {
                        (Some(_), Some(_)) if forked.contains(&at) => None,
                        (Some(host), Some(scope_host)) => {
                            forked.push(at);
                            later.push(scope_host);
                            Some(host)
                        }
                        (host, scope_host) => host.or(scope_host),
                    }
                }
            };
            match up.or_else(|| later.pop()) {
                Some(next) => at = next,
                None => return false,
            }
        }
    }

    /// The device px per logical px: the grid the published boxes are snapped to (1 until set).
    /// A scale that is not a positive finite number is ignored: the grid stays what it was.
    ///
    /// A new scale makes every root dirty ([`Tree::needs_layout_at`]), with a viewport or not: each is
    /// snapped again by its next layout, whenever that is.
    pub fn set_scale(&mut self, scale: f32) {
        if !(scale.is_finite() && scale > 0.0) || self.scale == scale {
            return;
        }
        self.scale = scale;
        for slot in 0..self.meta.len() {
            let meta = &self.meta[slot];
            if meta.alive && self.links[slot].parent.is_none() {
                self.mark_dirty(NodeId::new(slot, meta.generation));
            }
        }
    }

    // ── the pass ─────────────────────────────────────────────────────────────────────────────

    /// The next [`Tree::compute_layout`] has something to do: an edit reached a root with a
    /// viewport, or a style or a variable is waiting to be resolved (which may turn out to change
    /// nothing). `false`: the next pass changes nothing and costs nothing.
    pub fn needs_layout(&self) -> bool {
        self.needs_layout || !self.pending.is_empty() || !self.globals_changed.is_empty()
    }

    /// [`Tree::needs_layout`] about one subtree: something at or under `node` changed since its
    /// root was last laid out (an edit, a measured leaf marked dirty) — what an embedder that lays a
    /// root out on demand asks before it does. A variable changed in its scope counts only from the
    /// next pass's resolve.
    pub fn needs_layout_at(&self, node: NodeId) -> bool {
        self.meta[self.slot(node)].dirty
    }

    /// The pass: resolves the variables, lays out every root with a viewport that changed (hosts
    /// before the roots they host) and snaps the boxes to the device grid. The nodes whose published
    /// layout it changed — each once, a node published for the first time always — are then
    /// [`Tree::changes`], readable alongside [`Tree::layout`] until the next pass. A pass with
    /// nothing to do is free. The `measurer` is asked only about the measured leaves the pass lays
    /// out and has no cached answer for.
    pub fn compute_layout(&mut self, measurer: &mut dyn Measurer) {
        self.changes.clear();
        self.resolve_pending();
        if !self.needs_layout {
            return;
        }
        self.pass = self.pass.wrapping_add(1);
        for index in 0..self.roots.len() {
            self.compute_root(self.roots[index], measurer);
        }
        self.needs_layout = false;
    }

    fn compute_root(&mut self, root: NodeId, measurer: &mut dyn Measurer) {
        let slot = root.slot();
        if self.meta[slot].pass == self.pass {
            return;
        }
        self.meta[slot].pass = self.pass;

        // The box to lay out in, and where it is: the origin of the root's space, unrounded.
        let (size, origin) = match self.meta[slot].viewport.expect("a live root has a viewport") {
            Viewport::Size(size) => (size, (0.0, 0.0)),
            Viewport::Node(host) => {
                let host_root = self.root_of(host);
                if self.meta[host_root.slot()].viewport.is_some() {
                    self.compute_root(host_root, measurer);
                }
                let host_slot = host.slot();
                let NodeLayout { rect, padding, border, .. } = self.raw[host_slot];
                let left = border.left + padding.left;
                let top = border.top + padding.top;
                let width = (rect.width - left - border.right - padding.right).max(0.0);
                let height = (rect.height - top - border.bottom - padding.bottom).max(0.0);
                let host_origin = self.origin_of(host);
                (Size::new(Some(width), Some(height)), (host_origin.0 + left, host_origin.1 + top))
            }
        };

        if self.meta[slot].dirty || self.meta[slot].laid_in != Some(size) {
            algo::layout_root(self, root, size, measurer);
            self.meta[slot].laid_in = Some(size);
        }
        let device = (self.device(origin.0), self.device(origin.1));
        self.publish(root, origin, device);
    }

    /// The root of the node's own tree (hosts NOT followed).
    pub(crate) fn root_of(&self, node: NodeId) -> NodeId {
        let mut at = node;
        while let Some(parent) = self.links[at.slot()].parent {
            at = parent;
        }
        at
    }

    /// The node's unrounded origin in the space of its outermost root, hosts followed.
    fn origin_of(&self, node: NodeId) -> (f32, f32) {
        let mut origin = (0.0, 0.0);
        let mut at = node;
        loop {
            let slot = at.slot();
            let NodeLayout { rect, .. } = self.raw[slot];
            origin = (origin.0 + rect.x, origin.1 + rect.y);
            match (self.links[slot].parent, self.meta[slot].viewport) {
                (Some(parent), _) => at = parent,
                (None, Some(Viewport::Node(host))) => {
                    let NodeLayout { padding, border, .. } = self.raw[host.slot()];
                    origin = (origin.0 + border.left + padding.left, origin.1 + border.top + padding.top);
                    at = host;
                }
                _ => return origin,
            }
        }
    }

    /// A length snapped to the device grid, in logical px.
    fn snap(&self, value: f32) -> f32 {
        self.device(value) / self.scale
    }

    /// A logical position as the nearest whole device px.
    fn device(&self, value: f32) -> f32 {
        round_half_up(value * self.scale)
    }

    /// Turns the algorithm's `raw` boxes into the published ones: every edge snapped to the device
    /// grid by its ABSOLUTE position (so neighbours share an edge and no gap opens), the result
    /// diffed against the last pass. `parent` is the parent's origin in the root's space, unrounded;
    /// `parent_device` the same snapped, in whole device px.
    ///
    /// The edges are worked out in whole device px and a published length is one difference of
    /// them over the scale: exact (a difference of two lengths each snapped in logical px was off
    /// in its last bits, and by more the farther from the origin the box sat).
    fn publish(&mut self, node: NodeId, parent: (f32, f32), parent_device: (f32, f32)) {
        let slot = node.slot();
        let raw = self.raw[slot];
        let origin = (parent.0 + raw.rect.x, parent.1 + raw.rect.y);
        let left = self.device(origin.0);
        let top = self.device(origin.1);
        let (right, bottom) = (origin.0 + raw.rect.width, origin.1 + raw.rect.height);
        // A measured leaf never shrinks under what it measured: a text given a fraction of a px
        // less than it asked for breaks its last word. Its far edge is the snapped one, pushed out
        // only as far as the size takes from the snapped near edge — never further: a whole-px size
        // (a fixed `height`, a whole line box) stays that size wherever the leaf sits (a far edge
        // taken up to the grid gave a 40 px text at y 300.3 a box of 41).
        let (right, bottom) = if self.meta[slot].measured {
            let hold = |near: f32, far: f32, size: f32| self.device(far).max(near + (size * self.scale - 1e-3).ceil());
            (hold(left, right, raw.rect.width), hold(top, bottom, raw.rect.height))
        } else {
            (self.device(right), self.device(bottom))
        };
        let scale = self.scale;
        let new = NodeLayout {
            rect: Rect {
                x: (left - parent_device.0) / scale,
                y: (top - parent_device.1) / scale,
                width: (right - left).max(0.0) / scale,
                height: (bottom - top).max(0.0) / scale,
            },
            margin: raw.margin,
            padding: raw.padding,
            border: raw.border,
            content_size: Size::new(self.snap(raw.content_size.width), self.snap(raw.content_size.height)),
        }
        .finite();
        let old = self.out[slot];
        let first = !self.meta[slot].published;
        if new != old || first {
            self.out[slot] = new;
            self.meta[slot].published = true;
            self.changes.push(Change {
                node,
                moved: (new.rect.x, new.rect.y) != (old.rect.x, old.rect.y),
                resized: (new.rect.width, new.rect.height) != (old.rect.width, old.rect.height),
                inner: (new.margin, new.padding, new.border, new.content_size)
                    != (old.margin, old.padding, old.border, old.content_size),
            });
        }

        // The subtree is as it was unless the algorithm placed the children again, the node moved
        // by a fraction of a device pixel (a move by whole pixels shifts every snapped edge under it
        // by the same amount, and the boxes are relative), or the grid itself changed.
        //
        // Exact for every edge the snapping puts at the same pixel wherever the subtree sits, and
        // it does: a tie rounds toward +∞ (`round_half_up`), which commutes with a whole-pixel move
        // whatever the sign (a tie away from zero did not: an edge at −0.5 went to −1, at 0.5 to 1).
        // What is left is float error: an absolute position is a sum of offsets in f32, so an edge
        // within float error of a tie may land on the other side of it at the new place — one device
        // px from where a layout from scratch puts it (a published number, the difference of two
        // edges, two). Known, and the README says so.
        //
        // Past 2^24 device px an f32 holds no longer every whole pixel, and a move by whole pixels
        // is not one there: the phase of such an origin is NaN, which equals no other — the subtree
        // is published again on its way out of that range, and on its way in.
        const EXACT: f32 = 16_777_216.0;
        let device = (origin.0 * self.scale, origin.1 * self.scale);
        let phase = if device.0.abs() < EXACT && device.1.abs() < EXACT {
            (device.0.fract(), device.1.fract())
        } else {
            (f32::NAN, f32::NAN)
        };
        let meta = &mut self.meta[slot];
        let descend = meta.laid || meta.phase != phase || meta.scale != scale;
        meta.laid = false;
        meta.dirty = false;
        meta.phase = phase;
        meta.scale = scale;
        if descend {
            let mut child = self.links[slot].first_child;
            while let Some(id) = child {
                self.publish(id, origin, (left, top));
                child = self.links[id.slot()].next;
            }
        }
    }

    /// The nodes whose published layout the last [`Tree::compute_layout`] changed, each once (empty
    /// after a pass with nothing to do). The list stays as it is until the next pass.
    pub fn changes(&self) -> &[Change] {
        &self.changes
    }

    /// The node's published layout: what the last pass that reached it left.
    pub fn layout(&self, node: NodeId) -> &NodeLayout {
        &self.out[self.slot(node)]
    }
}

impl Meta {
    fn new(generation: NonZeroU32) -> Self {
        Self {
            generation,
            alive: true,
            measured: false,
            hug: false,
            dirty: true,
            dirtied: 0,
            resolved_in: 0,
            scope_resolved_in: 0,
            laid: false,
            uses: VarSet::EMPTY,
            subtree_uses: VarSet::EMPTY,
            defs: Vec::new(),
            hosts: false,
            scope_host: None,
            viewport: None,
            laid_in: None,
            pass: 0,
            phase: (0.0, 0.0),
            scale: 0.0,
            published: false,
        }
    }
}


#[cfg(test)]
mod tests {
    use super::round_half_up;

    #[test]
    fn a_tie_rounds_toward_positive_infinity_and_nothing_else_moves() {
        // The rule that commutes with a whole-pixel move: −0.5 and 0.5 one pixel apart, as before
        // the move.
        for (x, rounded) in [(-0.5, 0.0), (0.5, 1.0), (-1.5, -1.0), (1.5, 2.0), (-2.5, -2.0), (2.5, 3.0)] {
            assert_eq!(round_half_up(x), rounded, "{x}");
        }
        // Not a tie, either side of one: the nearest whole number.
        assert_eq!(round_half_up(-0.500_000_06), -1.0);
        assert_eq!(round_half_up(-0.499_999_97), 0.0);
        assert_eq!(round_half_up(0.3), 0.0);
        assert_eq!(round_half_up(-0.3), 0.0);
        // Where `(x + 0.5).floor()` is wrong in f32: the largest number under 0.5 (the sum rounds up
        // to 1.0), and an odd whole number past 2^23 (the sum rounds to the next even one).
        assert_eq!(round_half_up(0.499_999_97), 0.0);
        assert_eq!(round_half_up(8_388_609.0), 8_388_609.0);
        assert_eq!(round_half_up(-8_388_609.0), -8_388_609.0);
        assert!(round_half_up(f32::NAN).is_nan());
        assert_eq!(round_half_up(f32::INFINITY), f32::INFINITY);
        assert_eq!(round_half_up(f32::NEG_INFINITY), f32::NEG_INFINITY);
    }
}
