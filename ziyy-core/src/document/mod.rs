use std::fmt::{self, Debug, Display, Formatter};
use std::num::NonZeroU32;
use std::ops::{Index, IndexMut};

use crate::parser::Chunk;
use crate::{Tag, TagKind, TagName};

/// Ziyy Document
#[derive(Clone, PartialEq)]
pub struct Document<'src> {
    nodes: Vec<Node<'src>>,
}

/// Node ID.
///
/// Index into a `doc`-internal `Vec`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct NodeId(NonZeroU32);

impl NodeId {
    fn from_index(n: usize) -> Self {
        NodeId(match NonZeroU32::new(n as u32 + 1) {
            Some(n) => n,
            None => unreachable!(),
        })
    }

    fn to_index(self) -> usize {
        (self.0.get() - 1) as usize
    }
}

#[derive(Debug, Clone, PartialEq)]
struct Node<'src> {
    parent: Option<NodeId>,
    prev_sibling: Option<NodeId>,
    next_sibling: Option<NodeId>,
    children: Option<(NodeId, NodeId)>,
    value: Chunk<'src>,
}

impl<'src> Node<'src> {
    fn new(value: Chunk<'src>) -> Self {
        Node {
            parent: None,
            prev_sibling: None,
            next_sibling: None,
            children: None,
            value,
        }
    }
}

/// Node reference.
#[derive(Debug)]
pub struct NodeRef<'a, 'src> {
    /// Node ID.
    id: NodeId,

    /// doc containing the node.
    doc: &'a Document<'src>,

    node: &'a Node<'src>,
}

/// Node mutator.
#[derive(Debug)]
pub struct NodeMut<'a, 'src> {
    /// Node ID.
    id: NodeId,

    /// doc containing the node.
    doc: &'a mut Document<'src>,
}

// Trait implementations regardless of T.

impl Copy for NodeRef<'_, '_> {}
impl Clone for NodeRef<'_, '_> {
    fn clone(&self) -> Self {
        *self
    }
}

impl Eq for NodeRef<'_, '_> {}
impl PartialEq for NodeRef<'_, '_> {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
            && std::ptr::eq(self.doc, other.doc)
            && std::ptr::eq(self.node, other.node)
    }
}

impl<'src> Default for Document<'src> {
    fn default() -> Self {
        Self::new()
    }
}

impl<'src> Document<'src> {
    /// Creates a doc with a root node.
    #[must_use]
    pub fn new() -> Self {
        Document {
            nodes: vec![Node::new(Chunk::Tag(Tag::new(
                TagName::Any("$root"),
                TagKind::Open,
            )))],
        }
    }

    /// Creates a doc with a root node and the specified capacity.
    #[must_use]
    pub fn with_capacity(capacity: usize) -> Self {
        let mut vec = Vec::with_capacity(capacity);
        vec.push(Node::new(Chunk::Tag(Tag::new(
            TagName::Any("$root"),
            TagKind::Open,
        ))));
        Document { nodes: vec }
    }

    /// Returns a reference to the specified node.
    #[must_use]
    pub fn get(&self, id: NodeId) -> Option<NodeRef<'_, 'src>> {
        self.nodes.get(id.to_index()).map(|node| NodeRef {
            id,
            node,
            doc: self,
        })
    }

    /// Returns a mutator of the specified node.
    pub fn get_mut(&mut self, id: NodeId) -> Option<NodeMut<'_, 'src>> {
        let exists = self.nodes.get(id.to_index()).map(|_| ());
        exists.map(move |()| NodeMut { id, doc: self })
    }

    fn node(&self, id: NodeId) -> &Node<'src> {
        self.nodes.index(id.to_index())
    }

    fn node_mut(&mut self, id: NodeId) -> &mut Node<'src> {
        self.nodes.index_mut(id.to_index())
    }

    /// Returns a reference to the specified node.
    #[must_use]
    pub fn index(&self, id: NodeId) -> NodeRef<'_, 'src> {
        NodeRef {
            id,
            node: self.node(id),
            doc: self,
        }
    }

    /// Returns a mutator of the specified node.
    pub fn index_mut(&mut self, id: NodeId) -> NodeMut<'_, 'src> {
        NodeMut { id, doc: self }
    }

    /// Returns a reference to the root node.
    #[must_use]
    pub fn root(&self) -> NodeRef<'_, 'src> {
        self.index(NodeId::from_index(0))
    }

    /// Returns a mutator of the root node.
    pub fn root_mut(&mut self) -> NodeMut<'_, 'src> {
        self.index_mut(NodeId::from_index(0))
    }

    /// Creates an orphan node.
    pub fn orphan(&mut self, value: Chunk<'src>) -> NodeMut<'_, 'src> {
        let id = NodeId::from_index(self.nodes.len());
        self.nodes.push(Node::new(value));
        self.index_mut(id)
    }

    /// Merge with another doc as orphan, returning the new root of doc being merged.
    // Allowing this for compactness.
    #[allow(clippy::option_map_unit_fn)]
    pub fn extend_doc(&mut self, mut other_doc: Document<'src>) -> NodeMut<'_, 'src> {
        let offset = self.nodes.len();
        let offset_id = |id: NodeId| -> NodeId {
            let old_index = id.to_index();
            let new_index = old_index + offset;
            NodeId::from_index(new_index)
        };
        let other_doc_root_id = offset_id(other_doc.root().id);
        for node in &mut other_doc.nodes {
            node.parent.as_mut().map(|id| *id = offset_id(*id));
            node.prev_sibling.as_mut().map(|id| *id = offset_id(*id));
            node.next_sibling.as_mut().map(|id| *id = offset_id(*id));
            node.children.as_mut().map(|(id1, id2)| {
                *id1 = offset_id(*id1);
                *id2 = offset_id(*id2);
            });
        }
        self.nodes.append(&mut other_doc.nodes);
        self.index_mut(other_doc_root_id)
    }
}

impl<'a, 'src> NodeRef<'a, 'src> {
    /// Returns the ID of this node.
    #[must_use]
    pub fn id(&self) -> NodeId {
        self.id
    }

    /// Returns the doc owning this node.
    #[must_use]
    pub fn doc(&self) -> &'a Document<'src> {
        self.doc
    }

    /// Returns the value of this node.
    #[must_use]
    pub fn value(&self) -> &'a Chunk<'src> {
        &self.node.value
    }

    fn axis<F>(&self, f: F) -> Option<Self>
    where
        F: FnOnce(&Node<'src>) -> Option<NodeId>,
    {
        f(self.node).map(|id| self.doc.index(id))
    }

    /// Returns the parent of this node.
    #[must_use]
    pub fn parent(&self) -> Option<Self> {
        self.axis(|node| node.parent)
    }

    /// Returns the previous sibling of this node.
    #[must_use]
    pub fn prev_sibling(&self) -> Option<Self> {
        self.axis(|node| node.prev_sibling)
    }

    /// Returns the next sibling of this node.
    #[must_use]
    pub fn next_sibling(&self) -> Option<Self> {
        self.axis(|node| node.next_sibling)
    }

    /// Returns the first child of this node.
    #[must_use]
    pub fn first_child(&self) -> Option<Self> {
        self.axis(|node| node.children.map(|(id, _)| id))
    }

    /// Returns the last child of this node.
    #[must_use]
    pub fn last_child(&self) -> Option<Self> {
        self.axis(|node| node.children.map(|(_, id)| id))
    }

    /// Returns true if this node has siblings.
    #[must_use]
    pub fn has_siblings(&self) -> bool {
        self.node.prev_sibling.is_some() || self.node.next_sibling.is_some()
    }

    /// Returns true if this node has children.
    #[must_use]
    pub fn has_children(&self) -> bool {
        self.node.children.is_some()
    }
}

impl<'src> NodeMut<'_, 'src> {
    /// Returns the ID of this node.
    #[must_use]
    pub fn id(&self) -> NodeId {
        self.id
    }

    /// Returns the doc owning this node.
    pub fn doc(&mut self) -> &mut Document<'src> {
        self.doc
    }

    fn node(&mut self) -> &mut Node<'src> {
        self.doc.node_mut(self.id)
    }

    /// Returns the value of this node.
    pub fn value(&mut self) -> &mut Chunk<'src> {
        &mut self.node().value
    }

    /// Downcast `NodeMut` to `NodeRef`.
    pub fn as_ref(&mut self) -> NodeRef<'_, 'src> {
        self.doc.index(self.id)
    }

    fn axis<F>(&mut self, f: F) -> Option<NodeMut<'_, 'src>>
    where
        F: FnOnce(&mut Node<'src>) -> Option<NodeId>,
    {
        let id = f(self.node());
        id.map(move |id| self.doc.index_mut(id))
    }

    fn into_axis<F>(mut self, f: F) -> Result<Self, Self>
    where
        F: FnOnce(&mut Node<'src>) -> Option<NodeId>,
    {
        let id = f(self.node());
        match id {
            Some(id) => Ok(self.doc.index_mut(id)),
            None => Err(self),
        }
    }

    /// Returns the parent of this node.
    pub fn parent(&mut self) -> Option<NodeMut<'_, 'src>> {
        self.axis(|node| node.parent)
    }

    /// Returns the parent of this node.
    ///
    /// Returns `Ok(parent)` if possible and `Err(self)` otherwise
    /// so the caller can recover the current position.
    pub fn into_parent(self) -> Result<Self, Self> {
        self.into_axis(|node| node.parent)
    }

    /// Returns the previous sibling of this node.
    pub fn prev_sibling(&mut self) -> Option<NodeMut<'_, 'src>> {
        self.axis(|node| node.prev_sibling)
    }

    /// Returns the previous sibling of this node.
    ///
    /// Returns `Ok(prev_sibling)` if possible and `Err(self)` otherwise
    /// so the caller can recover the current position.
    pub fn into_prev_sibling(self) -> Result<Self, Self> {
        self.into_axis(|node| node.prev_sibling)
    }

    /// Returns the next sibling of this node.
    pub fn next_sibling(&mut self) -> Option<NodeMut<'_, 'src>> {
        self.axis(|node| node.next_sibling)
    }

    /// Returns the next sibling of this node.
    ///
    /// Returns `Ok(next_sibling)` if possible and `Err(self)` otherwise
    /// so the caller can recover the current position.
    pub fn into_next_sibling(self) -> Result<Self, Self> {
        self.into_axis(|node| node.next_sibling)
    }

    /// Returns the first child of this node.
    pub fn first_child(&mut self) -> Option<NodeMut<'_, 'src>> {
        self.axis(|node| node.children.map(|(id, _)| id))
    }

    /// Returns the first child of this node.
    ///
    /// Returns `Ok(first_child)` if possible and `Err(self)` otherwise
    /// so the caller can recover the current position.
    pub fn into_first_child(self) -> Result<Self, Self> {
        self.into_axis(|node| node.children.map(|(id, _)| id))
    }

    /// Returns the last child of this node.
    pub fn last_child(&mut self) -> Option<NodeMut<'_, 'src>> {
        self.axis(|node| node.children.map(|(_, id)| id))
    }

    /// Returns the last child of this node.
    ///
    /// Returns `Ok(last_child)` if possible and `Err(self)` otherwise
    /// so the caller can recover the current position.
    pub fn into_last_child(self) -> Result<Self, Self> {
        self.into_axis(|node| node.children.map(|(_, id)| id))
    }

    /// Returns true if this node has siblings.
    #[must_use]
    pub fn has_siblings(&self) -> bool {
        self.doc.index(self.id).has_siblings()
    }

    /// Returns true if this node has children.
    #[must_use]
    pub fn has_children(&self) -> bool {
        self.doc.index(self.id).has_children()
    }

    /// Apply function for each ancestor mutable node reference.
    pub fn for_each_ancestor<'b, F>(&'b mut self, mut f: F)
    where
        F: FnMut(&mut NodeMut<'b, 'src>),
    {
        let mut current = self.parent();
        while let Some(mut node) = current {
            f(&mut node);
            current = node.into_parent().ok();
        }
    }

    /// Apply function for each next sibling mutable node reference.
    pub fn for_each_next_sibling<'b, F>(&'b mut self, mut f: F)
    where
        F: FnMut(&mut NodeMut<'b, 'src>),
    {
        let mut current = self.next_sibling();
        while let Some(mut node) = current {
            f(&mut node);
            current = node.into_next_sibling().ok();
        }
    }

    /// Apply function for each previout sibling mutable node reference.
    pub fn for_each_prev_sibling<'b, F>(&'b mut self, mut f: F)
    where
        F: FnMut(&mut NodeMut<'b, 'src>),
    {
        let mut current = self.prev_sibling();
        while let Some(mut node) = current {
            f(&mut node);
            current = node.into_prev_sibling().ok();
        }
    }

    /// Apply function for this node and each sibling mutable node reference.
    pub fn for_each_sibling<F>(&mut self, mut f: F)
    where
        F: for<'b> FnMut(&mut NodeMut<'b, 'src>),
    {
        self.for_each_prev_sibling(&mut f);
        f(self);
        self.for_each_next_sibling(&mut f);
    }

    /// Apply function for each children mutable node reference.
    pub fn for_each_child<F>(&mut self, mut f: F)
    where
        F: for<'b> FnMut(&mut NodeMut<'b, 'src>),
    {
        let Some(mut first_child) = self.first_child() else {
            return;
        };
        f(&mut first_child);
        first_child.for_each_next_sibling(f);
    }

    /// Apply function for this node and each descendant mutable node reference.
    pub fn for_each_descendant<F>(&mut self, mut f: F)
    where
        F: FnMut(&mut NodeMut<'_, 'src>),
    {
        let id = self.id();

        f(self);

        // Start at our first child, if any.
        let Some(mut node) = self.first_child() else {
            return;
        };

        loop {
            f(&mut node);

            // Try to go deeper into its first child.
            match node.into_first_child() {
                Ok(child) => {
                    node = child;
                    continue;
                }
                Err(n) => {
                    node = n;
                }
            }

            // No deeper child, so climb until we find a next sibling or hit self.
            loop {
                match node.into_next_sibling() {
                    Ok(sib) => {
                        node = sib;
                        break;
                    }
                    Err(n) => {
                        node = n;
                    }
                }

                // No sibling, so climb up.
                let Ok(parent) = node.into_parent() else {
                    unreachable!();
                };
                if parent.id() == id {
                    return;
                }
                node = parent;
            }
        }
    }

    /// Appends a new child to this node.
    pub fn append(&mut self, value: Chunk<'src>) -> NodeMut<'_, 'src> {
        let id = self.doc.orphan(value).id;
        self.append_id(id)
    }

    /// Prepends a new child to this node.
    pub fn prepend(&mut self, value: Chunk<'src>) -> NodeMut<'_, 'src> {
        let id = self.doc.orphan(value).id;
        self.prepend_id(id)
    }

    /// Appends a subdoc, return the root of the merged subdoc.
    pub fn append_subdoc(&mut self, subdoc: Document<'src>) -> NodeMut<'_, 'src> {
        let root_id = self.doc.extend_doc(subdoc).id;
        self.append_id(root_id)
    }

    /// Prepends a subdoc, return the root of the merged subdoc.
    pub fn prepend_subdoc(&mut self, subdoc: Document<'src>) -> NodeMut<'_, 'src> {
        let root_id = self.doc.extend_doc(subdoc).id;
        self.prepend_id(root_id)
    }

    /// Inserts a new sibling before this node.
    ///
    /// # Panics
    ///
    /// Panics if this node is an orphan.
    pub fn insert_before(&mut self, value: Chunk<'src>) -> NodeMut<'_, 'src> {
        let id = self.doc.orphan(value).id;
        self.insert_id_before(id)
    }

    /// Inserts a new sibling after this node.
    ///
    /// # Panics
    ///
    /// Panics if this node is an orphan.
    pub fn insert_after(&mut self, value: Chunk<'src>) -> NodeMut<'_, 'src> {
        let id = self.doc.orphan(value).id;
        self.insert_id_after(id)
    }

    /// Detaches this node from its parent.
    pub fn detach(&mut self) {
        let parent_id = match self.node().parent {
            Some(id) => id,
            None => return,
        };
        let prev_sibling_id = self.node().prev_sibling;
        let next_sibling_id = self.node().next_sibling;

        {
            self.node().parent = None;
            self.node().prev_sibling = None;
            self.node().next_sibling = None;
        }

        if let Some(id) = prev_sibling_id {
            self.doc.node_mut(id).next_sibling = next_sibling_id;
        }
        if let Some(id) = next_sibling_id {
            self.doc.node_mut(id).prev_sibling = prev_sibling_id;
        }

        let parent = self.doc.node_mut(parent_id);
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
    ///
    /// # Panics
    ///
    /// Panics if `new_child_id` is not valid.
    pub fn append_id(&mut self, new_child_id: NodeId) -> NodeMut<'_, 'src> {
        assert_ne!(
            self.id(),
            new_child_id,
            "Cannot append node as a child to itself"
        );

        let last_child_id = self.node().children.map(|(_, id)| id);

        if last_child_id != Some(new_child_id) {
            {
                let mut new_child = self.doc.get_mut(new_child_id).unwrap();
                new_child.detach();
                new_child.node().parent = Some(self.id);
                new_child.node().prev_sibling = last_child_id;
            }

            if let Some(id) = last_child_id {
                self.doc.node_mut(id).next_sibling = Some(new_child_id);
            }

            {
                self.node().children = match self.node().children {
                    Some((first_child_id, _)) => Some((first_child_id, new_child_id)),
                    None => Some((new_child_id, new_child_id)),
                };
            }
        }

        self.doc.index_mut(new_child_id)
    }

    /// Prepends a child to this node.
    ///
    /// # Panics
    ///
    /// Panics if `new_child_id` is not valid.
    pub fn prepend_id(&mut self, new_child_id: NodeId) -> NodeMut<'_, 'src> {
        assert_ne!(
            self.id(),
            new_child_id,
            "Cannot prepend node as a child to itself"
        );

        let first_child_id = self.node().children.map(|(id, _)| id);

        if first_child_id != Some(new_child_id) {
            {
                let mut new_child = self.doc.get_mut(new_child_id).unwrap();
                new_child.detach();
                new_child.node().parent = Some(self.id);
                new_child.node().next_sibling = first_child_id;
            }

            if let Some(id) = first_child_id {
                self.doc.node_mut(id).prev_sibling = Some(new_child_id);
            }

            {
                self.node().children = match self.node().children {
                    Some((_, last_child_id)) => Some((new_child_id, last_child_id)),
                    None => Some((new_child_id, new_child_id)),
                };
            }
        }

        self.doc.index_mut(new_child_id)
    }

    /// Inserts a sibling before this node.
    ///
    /// # Panics
    ///
    /// - Panics if `new_sibling_id` is not valid.
    /// - Panics if this node is an orphan.
    pub fn insert_id_before(&mut self, new_sibling_id: NodeId) -> NodeMut<'_, 'src> {
        assert_ne!(
            self.id(),
            new_sibling_id,
            "Cannot insert node as a sibling of itself"
        );

        let parent_id = self.node().parent.unwrap();
        let prev_sibling_id = self.node().prev_sibling;

        {
            let mut new_sibling = self.doc.get_mut(new_sibling_id).unwrap();
            new_sibling.detach();
            new_sibling.node().parent = Some(parent_id);
            new_sibling.node().prev_sibling = prev_sibling_id;
            new_sibling.node().next_sibling = Some(self.id);
        }

        if let Some(id) = prev_sibling_id {
            self.doc.node_mut(id).next_sibling = Some(new_sibling_id);
        }

        self.node().prev_sibling = Some(new_sibling_id);

        {
            let parent = self.doc.node_mut(parent_id);
            let (first_child_id, last_child_id) = parent.children.unwrap();
            if first_child_id == self.id {
                parent.children = Some((new_sibling_id, last_child_id));
            }
        }

        self.doc.index_mut(new_sibling_id)
    }

    /// Inserts a sibling after this node.
    ///
    /// # Panics
    ///
    /// - Panics if `new_sibling_id` is not valid.
    /// - Panics if this node is an orphan.
    pub fn insert_id_after(&mut self, new_sibling_id: NodeId) -> NodeMut<'_, 'src> {
        assert_ne!(
            self.id(),
            new_sibling_id,
            "Cannot insert node as a sibling of itself"
        );

        let parent_id = self.node().parent.unwrap();
        let next_sibling_id = self.node().next_sibling;

        {
            let mut new_sibling = self.doc.get_mut(new_sibling_id).unwrap();
            new_sibling.detach();
            new_sibling.node().parent = Some(parent_id);
            new_sibling.node().prev_sibling = Some(self.id);
            new_sibling.node().next_sibling = next_sibling_id;
        }

        if let Some(id) = next_sibling_id {
            self.doc.node_mut(id).prev_sibling = Some(new_sibling_id);
        }

        self.node().next_sibling = Some(new_sibling_id);

        {
            let parent = self.doc.node_mut(parent_id);
            let (first_child_id, last_child_id) = parent.children.unwrap();
            if last_child_id == self.id {
                parent.children = Some((first_child_id, new_sibling_id));
            }
        }

        self.doc.index_mut(new_sibling_id)
    }

    /// Reparents the children of a node, appending them to this node.
    ///
    /// # Panics
    ///
    /// Panics if `from_id` is not valid.
    pub fn reparent_from_id_append(&mut self, from_id: NodeId) {
        assert_ne!(
            self.id(),
            from_id,
            "Cannot reparent node's children to itself"
        );

        let new_child_ids = {
            let mut from = self.doc.get_mut(from_id).unwrap();
            match from.node().children.take() {
                Some(ids) => ids,
                None => return,
            }
        };

        let mut child_id = new_child_ids.0;
        loop {
            let child = self.doc.node_mut(child_id);
            child.parent = Some(self.id);
            child_id = match child.next_sibling {
                Some(id) => id,
                None => break,
            };
        }

        if self.node().children.is_none() {
            self.node().children = Some(new_child_ids);
            return;
        }

        let old_child_ids = self.node().children.unwrap();

        self.doc.node_mut(old_child_ids.1).next_sibling = Some(new_child_ids.0);
        self.doc.node_mut(new_child_ids.0).prev_sibling = Some(old_child_ids.1);

        self.node().children = Some((old_child_ids.0, new_child_ids.1));
    }

    /// Reparents the children of a node, prepending them to this node.
    ///
    /// # Panics
    ///
    /// Panics if `from_id` is not valid.
    pub fn reparent_from_id_prepend(&mut self, from_id: NodeId) {
        assert_ne!(
            self.id(),
            from_id,
            "Cannot reparent node's children to itself"
        );

        let new_child_ids = {
            let mut from = self.doc.get_mut(from_id).unwrap();
            match from.node().children.take() {
                Some(ids) => ids,
                None => return,
            }
        };

        let mut child_id = new_child_ids.0;
        loop {
            let child = self.doc.node_mut(child_id);
            child.parent = Some(self.id);
            child_id = match child.next_sibling {
                Some(id) => id,
                None => break,
            };
        }

        if self.node().children.is_none() {
            self.node().children = Some(new_child_ids);
            return;
        }

        let old_child_ids = self.node().children.unwrap();
        self.doc.node_mut(old_child_ids.0).prev_sibling = Some(new_child_ids.1);
        self.doc.node_mut(new_child_ids.1).next_sibling = Some(old_child_ids.0);

        self.node().children = Some((new_child_ids.0, old_child_ids.1));
    }
}

impl<'a, 'src> From<NodeMut<'a, 'src>> for NodeRef<'a, 'src> {
    fn from(node: NodeMut<'a, 'src>) -> Self {
        node.doc.index(node.id)
    }
}

/// Iterators.
pub mod iter;

impl Debug for Document<'_> {
    fn fmt(&self, f: &mut Formatter) -> Result<(), fmt::Error> {
        use iter::Edge;
        if f.alternate() {
            write!(f, "doc {{")?;
            for edge in self.root().traverse() {
                match edge {
                    Edge::Open(node) if node.has_children() => {
                        write!(f, " {:?} => {{", node.value())?;
                    }
                    Edge::Open(node) if node.next_sibling().is_some() => {
                        write!(f, " {:?},", node.value())?;
                    }
                    Edge::Open(node) => {
                        write!(f, " {:?}", node.value())?;
                    }
                    Edge::Close(node) if node.has_children() => {
                        if node.next_sibling().is_some() {
                            write!(f, " }},")?;
                        } else {
                            write!(f, " }}")?;
                        }
                    }
                    _ => {}
                }
            }
            write!(f, " }}")
        } else {
            f.debug_struct("doc").field("vec", &self.nodes).finish()
        }
    }
}

// Handles display
mod display;

impl Display for Document<'_> {
    fn fmt(&self, f: &mut Formatter) -> Result<(), fmt::Error> {
        use display::Indentation;
        use iter::Edge;

        let mut indent: Indentation = Indentation::new(true);

        for edge in self.root().traverse() {
            match edge {
                Edge::Open(node) if node.has_children() => {
                    indent.indent(node.next_sibling().is_some());
                    writeln!(f, "{indent}{}", node.value())?;
                }
                Edge::Open(node) => {
                    indent.indent(node.next_sibling().is_some());
                    writeln!(f, "{indent}{}", node.value())?;
                    indent.deindent();
                }
                Edge::Close(node) if node.has_children() => {
                    indent.deindent();
                }
                _ => {}
            }
        }
        Ok(())
    }
}
