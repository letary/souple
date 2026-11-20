use std::num::NonZeroU32;

/// A node of a [`crate::Tree`]: a slot index and the slot's generation. A destroyed node's id is
/// stale — the slot is reused under the next generation — and every call that takes one panics on it
/// instead of touching another node ([`crate::Tree::is_alive`] asks first). An id is only
/// meaningful to the tree that made it.
#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub struct NodeId {
    index: u32,
    generation: NonZeroU32,
}

impl NodeId {
    pub(crate) fn new(index: usize, generation: NonZeroU32) -> Self {
        Self { index: index as u32, generation }
    }

    /// The slot: the index of this node in every per-node array.
    pub(crate) fn slot(self) -> usize {
        self.index as usize
    }

    pub(crate) fn generation(self) -> NonZeroU32 {
        self.generation
    }

    /// The node's slot: dense, reused after a destroy (under another generation). For an embedder
    /// that keeps its own per-node data in an array beside the tree.
    pub fn index(self) -> u32 {
        self.index
    }

    /// The id as one integer, for an embedder that keys a map by it or passes it over an ABI: the
    /// generation in the high 32 bits, the slot ([`NodeId::index`]) in the low 32.
    pub fn to_bits(self) -> u64 {
        (self.generation.get() as u64) << 32 | self.index as u64
    }

    /// The inverse of [`NodeId::to_bits`]; `None` for a value `to_bits` never made.
    pub fn from_bits(bits: u64) -> Option<Self> {
        let generation = NonZeroU32::new((bits >> 32) as u32)?;
        Some(Self { index: bits as u32, generation })
    }
}
