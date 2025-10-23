use std::iter::FusedIterator;
use std::ops::Range;
use std::{slice, vec};

use crate::parser::Chunk;

use super::{Document, Node, NodeId, NodeRef};

/// Iterator that moves out of a tree in insert order.
#[derive(Debug)]
pub struct IntoIter<'src>(vec::IntoIter<Node<'src>>);
impl ExactSizeIterator for IntoIter<'_> {}
impl FusedIterator for IntoIter<'_> {}
impl<'src> Iterator for IntoIter<'src> {
    type Item = Chunk<'src>;
    fn next(&mut self) -> Option<Self::Item> {
        self.0.next().map(|node| node.value)
    }
    fn size_hint(&self) -> (usize, Option<usize>) {
        self.0.size_hint()
    }
}
impl DoubleEndedIterator for IntoIter<'_> {
    fn next_back(&mut self) -> Option<Self::Item> {
        self.0.next_back().map(|node| node.value)
    }
}

/// Iterator over values in insert order.
#[derive(Debug)]
pub struct Values<'a, 'src>(slice::Iter<'a, Node<'src>>);
impl Clone for Values<'_, '_> {
    fn clone(&self) -> Self {
        Values(self.0.clone())
    }
}
impl ExactSizeIterator for Values<'_, '_> {}
impl FusedIterator for Values<'_, '_> {}
impl<'a, 'src> Iterator for Values<'a, 'src> {
    type Item = &'a Chunk<'src>;
    fn next(&mut self) -> Option<Self::Item> {
        self.0.next().map(|node| &node.value)
    }
    fn size_hint(&self) -> (usize, Option<usize>) {
        self.0.size_hint()
    }
}
impl DoubleEndedIterator for Values<'_, '_> {
    fn next_back(&mut self) -> Option<Self::Item> {
        self.0.next_back().map(|node| &node.value)
    }
}

/// Mutable iterator over values in insert order.
#[derive(Debug)]
pub struct ValuesMut<'a, 'src>(slice::IterMut<'a, Node<'src>>);
impl ExactSizeIterator for ValuesMut<'_, '_> {}
impl FusedIterator for ValuesMut<'_, '_> {}
impl<'a, 'src> Iterator for ValuesMut<'a, 'src> {
    type Item = &'a mut Chunk<'src>;
    fn next(&mut self) -> Option<Self::Item> {
        self.0.next().map(|node| &mut node.value)
    }
    fn size_hint(&self) -> (usize, Option<usize>) {
        self.0.size_hint()
    }
}
impl DoubleEndedIterator for ValuesMut<'_, '_> {
    fn next_back(&mut self) -> Option<Self::Item> {
        self.0.next_back().map(|node| &mut node.value)
    }
}

/// Iterator over nodes in insert order.
#[derive(Debug)]
pub struct Nodes<'a, 'src> {
    tree: &'a Document<'src>,
    iter: Range<usize>,
}
impl Clone for Nodes<'_, '_> {
    fn clone(&self) -> Self {
        Self {
            tree: self.tree,
            iter: self.iter.clone(),
        }
    }
}
impl ExactSizeIterator for Nodes<'_, '_> {}
impl FusedIterator for Nodes<'_, '_> {}
impl<'a, 'src> Iterator for Nodes<'a, 'src> {
    type Item = NodeRef<'a, 'src>;
    fn next(&mut self) -> Option<Self::Item> {
        self.iter
            .next()
            .map(|i| self.tree.index(NodeId::from_index(i)))
    }
    fn size_hint(&self) -> (usize, Option<usize>) {
        self.iter.size_hint()
    }
}
impl DoubleEndedIterator for Nodes<'_, '_> {
    fn next_back(&mut self) -> Option<Self::Item> {
        self.iter
            .next_back()
            .map(|i| self.tree.index(NodeId::from_index(i)))
    }
}

impl<'src> IntoIterator for Document<'src> {
    type Item = Chunk<'src>;
    type IntoIter = IntoIter<'src>;
    fn into_iter(self) -> Self::IntoIter {
        IntoIter(self.nodes.into_iter())
    }
}

impl<'src> Document<'src> {
    /// Returns an iterator over values in insert order.
    #[must_use]
    pub fn values(&self) -> Values<'_, 'src> {
        Values(self.nodes.iter())
    }

    /// Returns a mutable iterator over values in insert order.
    pub fn values_mut(&mut self) -> ValuesMut<'_, 'src> {
        ValuesMut(self.nodes.iter_mut())
    }

    /// Returns an iterator over nodes in insert order.
    #[must_use]
    pub fn nodes(&self) -> Nodes<'_, 'src> {
        Nodes {
            tree: self,
            iter: 0..self.nodes.len(),
        }
    }
}

macro_rules! axis_iterators {
    ($(#[$m:meta] $i:ident($f:path);)*) => {
        $(
            #[$m]
            #[derive(Debug)]
            pub struct $i<'a, 'src>(Option<NodeRef<'a, 'src>>);
            impl<'a, 'src> Clone for $i<'a, 'src> {
                fn clone(&self) -> Self {
                    $i(self.0)
                }
            }
            impl<'a, 'src> FusedIterator for $i<'a, 'src> {}
            impl<'a, 'src> Iterator for $i<'a, 'src> {
                type Item = NodeRef<'a, 'src>;
                fn next(&mut self) -> Option<Self::Item> {
                    let node = self.0.take();
                    self.0 = node.as_ref().and_then($f);
                    node
                }
            }
        )*
    };
}

axis_iterators! {
    /// Iterator over ancestors.
    Ancestors(NodeRef::parent);

    /// Iterator over previous siblings.
    PrevSiblings(NodeRef::prev_sibling);

    /// Iterator over next siblings.
    NextSiblings(NodeRef::next_sibling);

    /// Iterator over first children.
    FirstChildren(NodeRef::first_child);

    /// Iterator over last children.
    LastChildren(NodeRef::last_child);
}

/// Iterator over children.
#[derive(Debug)]
pub struct Children<'a, 'src> {
    front: Option<NodeRef<'a, 'src>>,
    back: Option<NodeRef<'a, 'src>>,
}
impl Clone for Children<'_, '_> {
    fn clone(&self) -> Self {
        Self {
            front: self.front,
            back: self.back,
        }
    }
}
impl FusedIterator for Children<'_, '_> {}
impl<'a, 'src> Iterator for Children<'a, 'src> {
    type Item = NodeRef<'a, 'src>;
    fn next(&mut self) -> Option<Self::Item> {
        if self.front == self.back {
            let node = self.front.take();
            self.back = None;
            node
        } else {
            let node = self.front.take();
            self.front = node.as_ref().and_then(NodeRef::next_sibling);
            node
        }
    }
}
impl DoubleEndedIterator for Children<'_, '_> {
    fn next_back(&mut self) -> Option<Self::Item> {
        if self.back == self.front {
            let node = self.back.take();
            self.front = None;
            node
        } else {
            let node = self.back.take();
            self.back = node.as_ref().and_then(NodeRef::prev_sibling);
            node
        }
    }
}

/// Open or close edge of a node.
#[derive(Debug)]
pub enum Edge<'a, 'src> {
    /// Open.
    Open(NodeRef<'a, 'src>),
    /// Close.
    Close(NodeRef<'a, 'src>),
}
impl Copy for Edge<'_, '_> {}
impl Clone for Edge<'_, '_> {
    fn clone(&self) -> Self {
        *self
    }
}
impl Eq for Edge<'_, '_> {}
impl PartialEq for Edge<'_, '_> {
    fn eq(&self, other: &Self) -> bool {
        match (*self, *other) {
            (Edge::Open(a), Edge::Open(b)) | (Edge::Close(a), Edge::Close(b)) => a == b,
            _ => false,
        }
    }
}

/// Iterator which traverses a subtree.
#[derive(Debug)]
pub struct Traverse<'a, 'src> {
    root: Option<NodeRef<'a, 'src>>,
    edge: Option<Edge<'a, 'src>>,
}
impl Clone for Traverse<'_, '_> {
    fn clone(&self) -> Self {
        Self {
            root: self.root,
            edge: self.edge,
        }
    }
}
impl FusedIterator for Traverse<'_, '_> {}
impl<'a, 'src> Iterator for Traverse<'a, 'src> {
    type Item = Edge<'a, 'src>;
    fn next(&mut self) -> Option<Self::Item> {
        match self.edge {
            None => {
                if let Some(root) = self.root {
                    self.edge = Some(Edge::Open(root));
                }
            }
            Some(Edge::Open(node)) => {
                if let Some(first_child) = node.first_child() {
                    self.edge = Some(Edge::Open(first_child));
                } else {
                    self.edge = Some(Edge::Close(node));
                }
            }
            Some(Edge::Close(node)) => {
                if node == self.root.unwrap() {
                    self.root = None;
                    self.edge = None;
                } else if let Some(next_sibling) = node.next_sibling() {
                    self.edge = Some(Edge::Open(next_sibling));
                } else {
                    self.edge = node.parent().map(Edge::Close);
                }
            }
        }
        self.edge
    }
}

/// Iterator over a node and its descendants.
#[derive(Debug)]
pub struct Descendants<'a, 'src>(Traverse<'a, 'src>);
impl Clone for Descendants<'_, '_> {
    fn clone(&self) -> Self {
        Descendants(self.0.clone())
    }
}
impl FusedIterator for Descendants<'_, '_> {}
impl<'a, 'src> Iterator for Descendants<'a, 'src> {
    type Item = NodeRef<'a, 'src>;
    fn next(&mut self) -> Option<Self::Item> {
        for edge in &mut self.0 {
            if let Edge::Open(node) = edge {
                return Some(node);
            }
        }
        None
    }
}

impl<'a, 'src> NodeRef<'a, 'src> {
    /// Returns an iterator over ancestors.
    #[must_use]
    pub fn ancestors(&self) -> Ancestors<'a, 'src> {
        Ancestors(self.parent())
    }

    /// Returns an iterator over previous siblings.
    #[must_use]
    pub fn prev_siblings(&self) -> PrevSiblings<'a, 'src> {
        PrevSiblings(self.prev_sibling())
    }

    /// Returns an iterator over next siblings.
    #[must_use]
    pub fn next_siblings(&self) -> NextSiblings<'a, 'src> {
        NextSiblings(self.next_sibling())
    }

    /// Returns an iterator over first children.
    #[must_use]
    pub fn first_children(&self) -> FirstChildren<'a, 'src> {
        FirstChildren(self.first_child())
    }

    /// Returns an iterator over last children.
    #[must_use]
    pub fn last_children(&self) -> LastChildren<'a, 'src> {
        LastChildren(self.last_child())
    }

    /// Returns an iterator over children.
    #[must_use]
    pub fn children(&self) -> Children<'a, 'src> {
        Children {
            front: self.first_child(),
            back: self.last_child(),
        }
    }

    /// Returns an iterator which traverses the subtree starting at this node.
    #[must_use]
    pub fn traverse(&self) -> Traverse<'a, 'src> {
        Traverse {
            root: Some(*self),
            edge: None,
        }
    }

    /// Returns an iterator over this node and its descendants.
    #[must_use]
    pub fn descendants(&self) -> Descendants<'a, 'src> {
        Descendants(self.traverse())
    }
}
