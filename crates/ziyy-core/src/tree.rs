use std::ops::{Deref, DerefMut};

use ego_tree::Tree as EgoTree;
pub use ego_tree::{NodeId, NodeMut, NodeRef, iter};

use crate::shared::Input;
use crate::{Chunk, Tag};

pub struct Tree<'src, I: ?Sized + Input> {
    tree: EgoTree<Chunk<'src, I>>,
}

impl<'src, I: ?Sized + Input> Tree<'src, I> {
    /// Creates a doc with a root node.
    #[must_use]
    pub fn new() -> Self {
        Tree {
            tree: EgoTree::new(Chunk::Tag(Tag::root())),
        }
    }

    /// Creates a doc with a root node and the specified capacity.
    #[must_use]
    pub fn with_capacity(capacity: usize) -> Self {
        Tree {
            tree: EgoTree::with_capacity(Chunk::Tag(Tag::root()), capacity),
        }
    }
}

impl<'src, I: ?Sized + Input> Default for Tree<'src, I> {
    fn default() -> Self {
        Tree::new()
    }
}

impl<'src, I: ?Sized + Input> Deref for Tree<'src, I> {
    type Target = EgoTree<Chunk<'src, I>>;

    fn deref(&self) -> &Self::Target {
        &self.tree
    }
}

impl<'src, I: ?Sized + Input> DerefMut for Tree<'src, I> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.tree
    }
}
