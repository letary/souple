//! Variables: numbers defined on nodes, read through the scope chain.
//!
//! A node may define any variable ([`Tree::set_var`]); a style's `Length::Var` reads the nearest
//! definition from the node up — its parents, then, for a hosted root, its host's chain (the host
//! of its `Viewport::Host`, else its scope host, [`Tree::set_scope_host`]) — and the tree's GLOBAL
//! value when nobody defines it ([`Tree::set_global`], 0 until set). So `vw` is a
//! global, `em` is defined on every node that sets a font size, `safe-bottom` is a global and is
//! defined again on the subtree a tab bar covers.
//!
//! The algorithm never sees a variable. A style is AUTHORED with them and COMPUTED without: this
//! module substitutes the values in scope, and a layout is redone only when a computed style
//! really changed. What needs resolving is queued (`pending`) and done at the start of a pass, so
//! a tree built bottom-up is resolved once, in its final place.

use crate::{EditError, NodeId, Tree, VarId, VarSet, Viewport};

/// A resolve to do at the next pass.
pub(crate) struct Pending {
    pub node: NodeId,
    /// The variables that may read differently.
    pub vars: VarSet,
    /// The node alone (its style was written) or everything in its scope (its place or a variable
    /// changed).
    pub subtree: bool,
}

impl Tree {
    /// Defines the variable on the node, for it and everything in its scope; `None` removes the
    /// definition (the value is the next one up again).
    pub fn set_var(&mut self, node: NodeId, var: VarId, value: Option<f32>) {
        let slot = self.slot(node);
        let defs = &mut self.meta[slot].defs;
        let at = defs.iter().position(|(defined, _)| *defined == var);
        match (at, value) {
            (Some(at), Some(value)) if defs[at].1 == value => return,
            (Some(at), Some(value)) => defs[at].1 = value,
            (Some(at), None) => {
                defs.swap_remove(at);
            }
            (None, Some(value)) => defs.push((var, value)),
            (None, None) => return,
        }
        self.pending.push(Pending { node, vars: VarSet::of(var), subtree: true });
    }

    /// The variable's value in the node's scope; its global value when no node up the chain
    /// defines it.
    pub fn var(&self, node: NodeId, var: VarId) -> f32 {
        self.slot(node);
        let mut at = Some(node);
        while let Some(id) = at {
            if let Some((_, value)) = self.meta[id.slot()].defs.iter().find(|(defined, _)| *defined == var) {
                return *value;
            }
            at = self.scope_parent(id);
        }
        self.globals[var.index()]
    }

    /// The value of the variable wherever no node up the chain defines it: a value of the whole
    /// tree (a viewport unit, a safe-area inset of the window), with no root to define it on — a
    /// node defining it shadows it for its subtree, as for any definition. The next pass resolves
    /// every style that reads it again.
    pub fn set_global(&mut self, var: VarId, value: f32) {
        // Bits, not `==`: a NaN set twice is the same value, and 0.0 / -0.0 are two.
        if self.globals[var.index()].to_bits() == value.to_bits() {
            return;
        }
        self.globals[var.index()] = value;
        self.globals_changed = self.globals_changed.union(VarSet::of(var));
    }

    /// The variable's global value ([`Tree::set_global`]).
    pub fn global(&self, var: VarId) -> f32 {
        self.globals[var.index()]
    }

    /// The next node up the scope chain: the parent, or a hosted root's host — its viewport's, else
    /// its scope host.
    pub(crate) fn scope_parent(&self, node: NodeId) -> Option<NodeId> {
        let slot = node.slot();
        self.links[slot].parent.or(match self.meta[slot].viewport {
            Some(Viewport::Host(host)) => Some(host),
            _ => self.meta[slot].scope_host,
        })
    }

    /// Continues a ROOT's scope chain at `host` (`None` ends it at the root again): the root and its
    /// subtree read the variables defined up the host's chain, as a root hosted by a
    /// `Viewport::Host` does — and that is ALL the link does. The root is laid out by whoever lays
    /// it out (its own viewport, given for a pass or kept), never in the host's box, and the host's
    /// layout never waits for it. What an embedder links this way is a root it hosts logically and
    /// lays out itself: a list's item, a widget declared in a screen, an inline run of a text.
    ///
    /// A child's chain is its parent's: inserting the root drops the link. While the root has a
    /// `Viewport::Host`, the chain goes through that host instead, and through this one again once
    /// the viewport goes.
    ///
    /// # Panics
    ///
    /// Where [`Tree::try_set_scope_host`] refuses: `root` has a parent, the chain would come back to
    /// the root — `host` is in the root's tree, or leads up to it by any link (parents, a root's
    /// viewport host or scope host) — or a node named was destroyed.
    pub fn set_scope_host(&mut self, root: NodeId, host: Option<NodeId>) {
        self.try_set_scope_host(root, host).unwrap_or_else(|error| panic!("Tree::set_scope_host: {error}"));
    }

    /// [`Tree::set_scope_host`], or why the tree refuses it — and then it changes nothing.
    pub fn try_set_scope_host(&mut self, root: NodeId, host: Option<NodeId>) -> Result<(), EditError> {
        let slot = self.live(root)?;
        if self.links[slot].parent.is_some() {
            return Err(EditError::NotARoot);
        }
        if let Some(host) = host {
            self.live(host)?;
            if self.leads_to(host, root) {
                return Err(EditError::Cycle);
            }
        }
        self.set_scope_host_unchecked(root, host);
        Ok(())
    }

    /// The root's scope host (`None` for a child and for a root without one).
    pub fn scope_host(&self, node: NodeId) -> Option<NodeId> {
        self.meta[self.slot(node)].scope_host
    }

    pub(crate) fn set_scope_host_unchecked(&mut self, root: NodeId, host: Option<NodeId>) {
        let slot = root.slot();
        if self.meta[slot].scope_host == host {
            return;
        }
        if self.meta[slot].scope_host.is_some() {
            self.scope_hosted.retain(|(_, hosted)| *hosted != root);
        }
        self.meta[slot].scope_host = host;
        if let Some(host) = host {
            self.scope_hosted.push((host, root));
            self.meta[host.slot()].hosts = true;
        }
        self.rescope_later(root);
    }

    /// Records that `vars` are read at or under `node`, up its scope chain.
    pub(crate) fn add_uses(&mut self, node: NodeId, vars: VarSet) {
        let mut at = Some(node);
        while let Some(id) = at {
            let meta = &mut self.meta[id.slot()];
            if meta.subtree_uses.includes(vars) {
                return;
            }
            meta.subtree_uses = meta.subtree_uses.union(vars);
            at = self.scope_parent(id);
        }
    }

    /// The node's scope chain changed (it was inserted, detached, hosted): resolve what it holds at
    /// the next pass.
    pub(crate) fn rescope_later(&mut self, node: NodeId) {
        let uses = self.meta[node.slot()].subtree_uses;
        if !uses.is_empty() {
            if let Some(up) = self.scope_parent(node) {
                self.add_uses(up, uses);
            }
            self.pending.push(Pending { node, vars: uses, subtree: true });
        }
    }

    pub(crate) fn resolve_pending(&mut self) {
        if self.pending.is_empty() && self.globals_changed.is_empty() {
            return;
        }
        // One sweep: the tree and the values do not change during it, so a style is computed once
        // however many queued entries reach it (a tree built children-first queues one per insert).
        self.sweep = self.sweep.wrapping_add(1);
        // A global is read wherever no definition shadows it — in any tree of the forest, a root
        // without a viewport included — so its readers are found by their slots, not by a walk
        // from roots the engine may not know (`resolve_node` reads the scope, a reader under a
        // definition computes what it had).
        let globals = std::mem::replace(&mut self.globals_changed, VarSet::EMPTY);
        if !globals.is_empty() {
            for slot in 0..self.meta.len() {
                let meta = &self.meta[slot];
                if meta.alive && meta.uses.intersects(globals) {
                    self.resolve_node(NodeId::new(slot, meta.generation));
                }
            }
        }
        // A resolve queues nothing, so one sweep empties the list.
        for Pending { node, vars, subtree } in std::mem::take(&mut self.pending) {
            if !self.is_alive(node) {
                continue;
            }
            if subtree {
                self.resolve_scope(node, vars, true);
            } else {
                self.resolve_node(node);
            }
        }
    }

    /// Resolves every style in `node`'s scope that reads one of `vars`. A node below the first that
    /// defines a variable itself shadows it: the walk drops it there, and skips whole subtrees that
    /// read none of what is left.
    fn resolve_scope(&mut self, node: NodeId, vars: VarSet, first: bool) {
        let slot = node.slot();
        if self.meta[slot].scope_resolved_in == self.sweep {
            return;
        }
        let mut vars = vars;
        if !first {
            for (defined, _) in &self.meta[slot].defs {
                vars = vars.without(VarSet::of(*defined));
            }
        }
        if !self.meta[slot].subtree_uses.intersects(vars) {
            return;
        }
        if self.meta[slot].uses.intersects(vars) {
            self.resolve_node(node);
        }
        let mut child = self.links[slot].first_child;
        while let Some(id) = child {
            self.resolve_scope(id, vars, false);
            child = self.links[id.slot()].next;
        }
        if self.meta[slot].hosts {
            let roots: Vec<NodeId> = self
                .hosted
                .iter()
                .chain(&self.scope_hosted)
                .filter(|(host, _)| *host == node)
                .map(|(_, root)| *root)
                .collect();
            for root in roots {
                self.resolve_scope(root, vars, false);
            }
        }
        // The walk came with every variable this subtree reads from outside: all of it is resolved
        // against the final values, and a later entry of this sweep can stop here.
        if vars.includes(self.meta[slot].subtree_uses) {
            self.meta[slot].scope_resolved_in = self.sweep;
        }
    }

    /// Computes the node's style from the authored one and the values in scope.
    fn resolve_node(&mut self, node: NodeId) {
        let slot = node.slot();
        if self.meta[slot].uses.is_empty() || self.meta[slot].resolved_in == self.sweep {
            return;
        }
        self.meta[slot].resolved_in = self.sweep;
        let computed = self.styles[slot].authored.substitute(&|var| self.var(node, var));
        if self.styles[slot].resolved.as_deref() != Some(&computed) {
            self.styles[slot].resolved = Some(Box::new(computed));
            self.mark_dirty(node);
        }
    }
}
