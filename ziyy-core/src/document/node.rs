use std::num::NonZeroU32;
use std::rc::{Rc, Weak};

use crate::parser::Chunk;

use super::Document;

pub type NodeId = NonZeroU32;

#[derive(Debug, Clone)]
pub struct Node<'src> {
    id: NodeId,
    parent: Option<NodeId>,
    prev_sibling: Option<NodeId>,
    next_sibling: Option<NodeId>,
    children: Option<(NodeId, NodeId)>,
    pub(crate) chunk: Chunk<'src>,
}

impl<'src> Node<'src> {
    pub fn new(id: NodeId, chunk: Chunk<'src>) -> Self {
        Self {
            id,
            parent: None,
            prev_sibling: None,
            next_sibling: None,
            children: None,
            doc,
            chunk,
        }
    }

    pub fn doc(&self) -> Rc<Document<'src>> {
        self.doc.upgrade().unwrap()
    }

    /// Returns the chunk of this node.
    pub fn chunk(&self) -> &Chunk<'src> {
        &self.chunk
    }

    fn axis<F>(&self, f: F) -> Option<Self>
    where
        F: FnOnce(&Node<'src>) -> Option<NodeId>,
    {
        f(self).map(|id| self.doc().get(id))
    }

    /// Returns the parent of this node.
    pub fn parent(&self) -> Option<Self> {
        self.axis(|node| node.parent)
    }

    /// Returns the previous sibling of this node.
    pub fn prev_sibling(&self) -> Option<Self> {
        self.axis(|node| node.prev_sibling)
    }

    /// Returns the next sibling of this node.
    pub fn next_sibling(&self) -> Option<Self> {
        self.axis(|node| node.next_sibling)
    }

    /// Returns the first child of this node.
    pub fn first_child(&self) -> Option<Self> {
        self.axis(|node| node.children.map(|(id, _)| id))
    }

    /// Returns the last child of this node.
    pub fn last_child(&self) -> Option<Self> {
        self.axis(|node| node.children.map(|(_, id)| id))
    }

    /// Returns true if this node has children.
    pub fn has_children(&self) -> bool {
        self.children.is_some()
    }

    /// Appends a new child to this node.
    pub fn append(&mut self, value: Chunk<'src>) -> Self {
        let id = self.doc().orphan(value).id;
        self.append_id(id)
    }

    /// Prepends a new child to this node.
    #[allow(dead_code)]
    pub fn prepend(&mut self, value: Chunk<'src>) -> Self {
        let id = self.doc().orphan(value).id;
        self.prepend_id(id)
    }

    /// Inserts a new sibling before this node.
    ///
    /// # Panics
    ///
    /// Panics if this node is an orphan.
    pub fn insert_before(&mut self, value: Chunk<'src>) -> Self {
        let id = self.doc().orphan(value).id;
        self.insert_id_before(id)
    }

    /// Inserts a new sibling after this node.
    ///
    /// # Panics
    ///
    /// Panics if this node is an orphan.
    pub fn insert_after(&mut self, value: Chunk<'src>) -> Self {
        let id = self.doc().orphan(value).id;
        self.insert_id_after(id)
    }

    /// Detaches this node from its parent.
    pub fn detach(&mut self) {
        let parent_id = match self.parent {
            Some(id) => id,
            None => return,
        };
        let prev_sibling_id = self.prev_sibling;
        let next_sibling_id = self.next_sibling;

        {
            self.parent = None;
            self.prev_sibling = None;
            self.next_sibling = None;
        }

        if let Some(id) = prev_sibling_id {
            self.doc().node(id).next_sibling = next_sibling_id;
        }
        if let Some(id) = next_sibling_id {
            self.doc().node(id).prev_sibling = prev_sibling_id;
        }

        let doc = self.doc();
        let mut parent = doc.node(parent_id);
        let (first_child_id, last_child_id) = parent.children.unwrap();
        if first_child_id == last_child_id {
            parent.children = None;
        } else if first_child_id == self.id {
            parent.children = Some((next_sibling_id.unwrap(), last_child_id));
        } else if last_child_id == self.id {
            parent.children = Some((first_child_id, prev_sibling_id.unwrap()));
        }
    }

    /// Appends a child to this node.
    pub fn append_id(&mut self, new_child_id: NodeId) -> Self {
        assert_ne!(
            self.id, new_child_id,
            "Cannot append node as a child to itself"
        );

        let last_child_id = self.children.map(|(_, id)| id);

        if last_child_id != Some(new_child_id) {
            {
                let mut new_child = self.doc().get(new_child_id);
                new_child.detach();
                new_child.parent = Some(self.id);
                new_child.prev_sibling = last_child_id;
            }

            if let Some(id) = last_child_id {
                self.doc().node(id).next_sibling = Some(new_child_id);
            }

            self.children = match self.children {
                Some((first_child_id, _)) => Some((first_child_id, new_child_id)),
                None => Some((new_child_id, new_child_id)),
            };
        }

        self.doc().get(new_child_id)
    }

    /// Prepends a child to this node.
    #[allow(dead_code)]
    pub fn prepend_id(&mut self, new_child_id: NodeId) -> Self {
        assert_ne!(
            self.id, new_child_id,
            "Cannot prepend node as a child to itself"
        );

        let first_child_id = self.children.map(|(id, _)| id);

        if first_child_id != Some(new_child_id) {
            let mut new_child = self.doc().get(new_child_id);
            new_child.detach();
            new_child.parent = Some(self.id);
            new_child.next_sibling = first_child_id;

            if let Some(id) = first_child_id {
                self.doc().node(id).prev_sibling = Some(new_child_id);
            }

            self.children = match self.children {
                Some((_, last_child_id)) => Some((new_child_id, last_child_id)),
                None => Some((new_child_id, new_child_id)),
            };
        }

        self.doc().get(new_child_id)
    }

    /// Inserts a sibling before this node.
    ///
    /// # Panics
    ///
    /// - Panics if `new_sibling_id` is not valid.
    /// - Panics if this node is an orphan.
    pub fn insert_id_before(&mut self, new_sibling_id: NodeId) -> Self {
        assert_ne!(
            self.id, new_sibling_id,
            "Cannot insert node as a sibling of itself"
        );

        let parent_id = self.parent.unwrap();
        let prev_sibling_id = self.prev_sibling;

        {
            let mut new_sibling = self.doc().get(new_sibling_id);
            new_sibling.detach();
            new_sibling.parent = Some(parent_id);
            new_sibling.prev_sibling = prev_sibling_id;
            new_sibling.next_sibling = Some(self.id);
        }

        if let Some(id) = prev_sibling_id {
            self.doc().node(id).next_sibling = Some(new_sibling_id);
        }

        self.prev_sibling = Some(new_sibling_id);

        {
            let doc = self.doc();
            let mut parent = doc.node(parent_id);
            let (first_child_id, last_child_id) = parent.children.unwrap();
            if first_child_id == self.id {
                parent.children = Some((new_sibling_id, last_child_id));
            }
        }

        self.doc().get(new_sibling_id)
    }

    /// Inserts a sibling after this node.
    ///
    /// # Panics
    ///
    /// - Panics if `new_sibling_id` is not valid.
    /// - Panics if this node is an orphan.
    pub fn insert_id_after(&mut self, new_sibling_id: NodeId) -> Self {
        assert_ne!(
            self.id, new_sibling_id,
            "Cannot insert node as a sibling of itself"
        );

        let parent_id = self.parent.unwrap();
        let next_sibling_id = self.next_sibling;

        {
            let mut new_sibling = self.doc().get(new_sibling_id);
            new_sibling.detach();
            new_sibling.parent = Some(parent_id);
            new_sibling.prev_sibling = Some(self.id);
            new_sibling.next_sibling = next_sibling_id;
        }

        if let Some(id) = next_sibling_id {
            self.doc().node(id).prev_sibling = Some(new_sibling_id);
        }

        self.next_sibling = Some(new_sibling_id);

        {
            let doc = self.doc();
            let mut parent = doc.node(parent_id);
            let (first_child_id, last_child_id) = parent.children.unwrap();
            if last_child_id == self.id {
                parent.children = Some((first_child_id, new_sibling_id));
            }
        }

        self.doc().get(new_sibling_id)
    }
}

impl<'src> PartialEq for Node<'src> {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
            && self.parent == other.parent
            && self.prev_sibling == other.prev_sibling
            && self.next_sibling == other.next_sibling
            && self.children == other.children
            && self.chunk == other.chunk
    }
}

impl<'src> Eq for Node<'src> {}
