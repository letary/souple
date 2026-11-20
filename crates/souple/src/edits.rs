//! The tree: each node links its parent, its first and last child and its two siblings, so an
//! insert, a move and a detach are O(1) and a child list needs no allocation.

use crate::{Tree, NodeId};

#[derive(Clone, Copy, Default)]
pub(crate) struct Links {
    pub parent: Option<NodeId>,
    pub first_child: Option<NodeId>,
    pub last_child: Option<NodeId>,
    pub prev: Option<NodeId>,
    pub next: Option<NodeId>,
}

impl Tree {
    /// Puts `child` under `parent`, before the sibling `before` (`None` = last). A child that has a
    /// parent already is MOVED — within its parent or to another — in this one call. A root's scope
    /// host is dropped: a child's scope is its parent's.
    ///
    /// # Panics
    ///
    /// If `child` has a viewport (take it away first: [`Tree::clear_viewport`]), `before` is
    /// `child` or not a child of `parent`, or `parent` is `child` or under it — along the scope
    /// chain too (a host inserted under a root it hosts).
    pub fn insert(&mut self, parent: NodeId, child: NodeId, before: Option<NodeId>) {
        self.slot(parent);
        let child_slot = self.slot(child);
        assert!(self.meta[child_slot].viewport.is_none(), "a root with a viewport cannot become a child");
        assert!(before != Some(child), "a node cannot be inserted before itself");
        // Not under itself — along every link of the scope chain too: a host inserted under a root
        // it hosts would make the chain a loop.
        assert!(!self.leads_to(parent, child), "a node cannot be inserted under itself");
        if let Some(before) = before {
            assert!(self.links[self.slot(before)].parent == Some(parent), "`before` is a child of `parent`");
        }

        if let Some(old_parent) = self.links[child_slot].parent {
            self.unlink(child);
            self.mark_dirty(old_parent);
        }
        // A child's scope is its parent's (the rescope below resolves it there).
        if self.meta[child_slot].scope_host.is_some() {
            self.scope_hosted.retain(|(_, hosted)| *hosted != child);
            self.meta[child_slot].scope_host = None;
        }

        let prev = match before {
            Some(before) => self.links[before.slot()].prev,
            None => self.links[parent.slot()].last_child,
        };
        self.links[child_slot] = Links { parent: Some(parent), prev, next: before, ..self.links[child_slot] };
        match prev {
            Some(prev) => self.links[prev.slot()].next = Some(child),
            None => self.links[parent.slot()].first_child = Some(child),
        }
        match before {
            Some(before) => self.links[before.slot()].prev = Some(child),
            None => self.links[parent.slot()].last_child = Some(child),
        }

        self.mark_dirty(parent);
        self.add_uses(parent, self.meta[child_slot].subtree_uses);
        self.rescope_later(child);
    }

    /// [`Tree::insert`] as the last child.
    pub fn append(&mut self, parent: NodeId, child: NodeId) {
        self.insert(parent, child, None);
    }

    /// Takes the node out of its parent: it is a root again, with its subtree.
    pub fn detach(&mut self, child: NodeId) {
        let slot = self.slot(child);
        let Some(parent) = self.links[slot].parent else { return };
        self.unlink(child);
        self.mark_dirty(parent);
        self.mark_dirty(child);
        self.rescope_later(child);
    }

    fn unlink(&mut self, child: NodeId) {
        let Links { parent, prev, next, .. } = self.links[child.slot()];
        let parent = parent.expect("unlink: the node has a parent").slot();
        match prev {
            Some(prev) => self.links[prev.slot()].next = next,
            None => self.links[parent].first_child = next,
        }
        match next {
            Some(next) => self.links[next.slot()].prev = prev,
            None => self.links[parent].last_child = prev,
        }
        let links = &mut self.links[child.slot()];
        links.parent = None;
        links.prev = None;
        links.next = None;
    }

    /// The node's parent; `None` for a root (a hosted root's host is not its parent).
    pub fn parent(&self, node: NodeId) -> Option<NodeId> {
        self.links[self.slot(node)].parent
    }

    /// The node's children, first to last.
    pub fn children(&self, node: NodeId) -> impl Iterator<Item = NodeId> + '_ {
        let mut next = self.links[self.slot(node)].first_child;
        std::iter::from_fn(move || {
            let child = next?;
            next = self.links[child.slot()].next;
            Some(child)
        })
    }

    /// How many children the node has (a walk over them: O(children)).
    pub fn child_count(&self, node: NodeId) -> usize {
        self.children(node).count()
    }

    /// The node's next sibling, `None` for the last child (and a root): a walk over the children
    /// that keeps its place between calls (an embedder answering "child i" for i = 0, 1, 2 …).
    pub fn next_sibling(&self, node: NodeId) -> Option<NodeId> {
        self.links[self.slot(node)].next
    }
}
