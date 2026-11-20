//! The tree: each node links its parent, its first and last child and its two siblings, and counts
//! its children, so an insert, a move, a detach and a child count are O(1) and a child list needs
//! no allocation.

use crate::{NodeId, Tree};

/// Why the tree refused an edit: the answer of the `try_` edits ([`Tree::try_insert`],
/// [`Tree::try_append`], [`Tree::try_set_viewport`], [`Tree::try_set_scope_host`]). A refused edit
/// changes nothing — boxes, dirtiness, the next pass are what they were; the plain forms panic
/// with it.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[non_exhaustive]
pub enum EditError {
    /// A node named was destroyed: its `NodeId` is stale.
    Destroyed,
    /// The edit would close a loop: the new parent is the child or under it (an insert), or the
    /// host leads back up to the root — by parents, a root's viewport host or its scope host (a
    /// `Viewport::Host`, a scope host). A host inserted under a root it hosts is one too.
    Cycle,
    /// A root with a viewport cannot become a child (an insert): clear the viewport first.
    HasViewport,
    /// Only a root has a viewport or a scope host: the node has a parent.
    NotARoot,
    /// An insert's `before` is not a child of the parent, or is the child itself.
    NotASibling,
}

impl std::fmt::Display for EditError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(match self {
            Self::Destroyed => "a stale NodeId: the node was destroyed",
            Self::Cycle => "a node cannot be under itself: the parent, or the host, is in its subtree or leads back up to it",
            Self::HasViewport => "a root with a viewport cannot become a child (clear the viewport first)",
            Self::NotARoot => "only a root has a viewport or a scope host",
            Self::NotASibling => "`before` is not a child of the parent, or is the child itself",
        })
    }
}

impl std::error::Error for EditError {}

#[derive(Clone, Copy, Default)]
pub(crate) struct Links {
    pub parent: Option<NodeId>,
    pub first_child: Option<NodeId>,
    pub last_child: Option<NodeId>,
    pub prev: Option<NodeId>,
    pub next: Option<NodeId>,
    /// How many children: kept by `insert` and `unlink`, the only edits of a child list.
    pub children: u32,
}

impl Tree {
    /// Puts `child` under `parent`, before the sibling `before` (`None` = last). A child that has a
    /// parent already is MOVED — within its parent or to another — in this one call. A root's scope
    /// host is dropped: a child's scope is its parent's.
    ///
    /// # Panics
    ///
    /// Where [`Tree::try_insert`] refuses: `child` has a viewport (take it away first:
    /// [`Tree::clear_viewport`]), `before` is `child` or not a child of `parent`, `parent` is `child`
    /// or under it — along the scope chain too (a host inserted under a root it hosts) — or a node
    /// named was destroyed.
    pub fn insert(&mut self, parent: NodeId, child: NodeId, before: Option<NodeId>) {
        self.try_insert(parent, child, before).unwrap_or_else(|error| panic!("Tree::insert: {error}"));
    }

    /// [`Tree::insert`], or why the tree refuses it — and then it changes nothing. For an embedder
    /// whose edits come from elsewhere (an app, a host) and that refuses a bad one rather than
    /// stops: a loop of parents and hosts is only found by walking them.
    pub fn try_insert(&mut self, parent: NodeId, child: NodeId, before: Option<NodeId>) -> Result<(), EditError> {
        self.live(parent)?;
        let child_slot = self.live(child)?;
        if self.meta[child_slot].viewport.is_some() {
            return Err(EditError::HasViewport);
        }
        if before == Some(child) {
            return Err(EditError::NotASibling);
        }
        // Not under itself — along every link of the scope chain too: a host inserted under a root
        // it hosts would make the chain a loop.
        if self.leads_to(parent, child) {
            return Err(EditError::Cycle);
        }
        if let Some(before) = before
            && self.links[self.live(before)?].parent != Some(parent)
        {
            return Err(EditError::NotASibling);
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
        self.links[parent.slot()].children += 1;

        self.mark_dirty(parent);
        self.add_uses(parent, self.meta[child_slot].subtree_uses);
        self.rescope_later(child);
        Ok(())
    }

    /// [`Tree::insert`] as the last child.
    pub fn append(&mut self, parent: NodeId, child: NodeId) {
        self.insert(parent, child, None);
    }

    /// [`Tree::try_insert`] as the last child.
    pub fn try_append(&mut self, parent: NodeId, child: NodeId) -> Result<(), EditError> {
        self.try_insert(parent, child, None)
    }

    /// The node's slot, or [`EditError::Destroyed`].
    pub(crate) fn live(&self, node: NodeId) -> Result<usize, EditError> {
        if self.is_alive(node) { Ok(node.slot()) } else { Err(EditError::Destroyed) }
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
        self.links[parent].children -= 1;
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

    /// How many children the node has (counted as they come and go: O(1)).
    pub fn child_count(&self, node: NodeId) -> usize {
        self.links[self.slot(node)].children as usize
    }

    /// The node's next sibling, `None` for the last child (and a root): a walk over the children
    /// that keeps its place between calls (an embedder answering "child i" for i = 0, 1, 2 …).
    pub fn next_sibling(&self, node: NodeId) -> Option<NodeId> {
        self.links[self.slot(node)].next
    }
}
