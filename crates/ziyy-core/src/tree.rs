use std::ops::{Deref, DerefMut};

use ego_tree::Tree;
pub use ego_tree::{iter, NodeId, NodeMut, NodeRef};

use crate::shared::Input;
use crate::{Chunk, Tag};

pub struct Document<'src, I: ?Sized + Input> {
    tree: Tree<Chunk<'src, I>>,
}

impl<'src, I: ?Sized + Input> Document<'src, I> {
    /// Creates a doc with a root node.
    #[must_use]
    pub fn new() -> Self {
        Document {
            tree: Tree::new(Chunk::Tag(Tag::root())),
        }
    }

    /// Creates a doc with a root node and the specified capacity.
    #[must_use]
    pub fn with_capacity(capacity: usize) -> Self {
        Document {
            tree: Tree::with_capacity(Chunk::Tag(Tag::root()), capacity),
        }
    }
}

impl<'src, I: ?Sized + Input> Deref for Document<'src, I> {
    type Target = Tree<Chunk<'src, I>>;

    fn deref(&self) -> &Self::Target {
        &self.tree
    }
}

impl<'src, I: ?Sized + Input> DerefMut for Document<'src, I> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.tree
    }
}
