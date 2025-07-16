use std::{ops::Deref, rc::Rc};

use super::Node;

#[derive(Debug)]
pub struct Children<'a> {
    front: Option<Rc<Node<'a>>>,
    back: Option<Rc<Node<'a>>>,
}

impl<'a> Clone for Children<'a> {
    fn clone(&self) -> Self {
        Self {
            front: self.front.clone(),
            back: self.back.clone(),
        }
    }
}

impl<'a> Iterator for Children<'a> {
    type Item = Rc<Node<'a>>;
    fn next(&mut self) -> Option<Self::Item> {
        if self.front == self.back {
            let node = self.front.take();
            self.back = None;
            node
        } else {
            let node = self.front.take();
            self.front = node
                .as_ref()
                .map(|x| x.deref())
                .and_then(Node::next_sibling);
            node
        }
    }
}

/// Open or close edge of a node.
#[derive(Debug, Clone)]
pub enum Edge<'a> {
    /// Open.
    Open(Rc<Node<'a>>),
    /// Close.
    Close(Rc<Node<'a>>),
}

impl<'a> Eq for Edge<'a> {}

impl<'a> PartialEq for Edge<'a> {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Edge::Open(a), Edge::Open(b)) | (Edge::Close(a), Edge::Close(b)) => a == b,
            _ => false,
        }
    }
}

/// Iterator which traverses a subtree.
#[derive(Debug, Clone)]
pub struct Traverse<'a> {
    root: Option<Rc<Node<'a>>>,
    edge: Option<Edge<'a>>,
}

impl<'a> Iterator for Traverse<'a> {
    type Item = Edge<'a>;
    fn next(&mut self) -> Option<Self::Item> {
        match &self.edge {
            None => {
                if let Some(root) = &self.root {
                    self.edge = Some(Edge::Open(root.clone()));
                }
            }
            Some(Edge::Open(node)) => {
                if let Some(first_child) = node.first_child() {
                    self.edge = Some(Edge::Open(first_child));
                } else {
                    self.edge = Some(Edge::Close(node.clone()));
                }
            }
            Some(Edge::Close(node)) => {
                if *node == self.root.clone().unwrap() {
                    self.root = None;
                    self.edge = None;
                } else if let Some(next_sibling) = node.next_sibling() {
                    self.edge = Some(Edge::Open(next_sibling));
                } else {
                    self.edge = node.parent().map(Edge::Close);
                }
            }
        }
        self.edge.clone()
    }
}

/// Iterator over a node and its descendants.
#[derive(Debug)]
pub struct Descendants<'a>(Traverse<'a>);

impl<'a> Clone for Descendants<'a> {
    fn clone(&self) -> Self {
        Descendants(self.0.clone())
    }
}

impl<'a> Iterator for Descendants<'a> {
    type Item = Rc<Node<'a>>;
    fn next(&mut self) -> Option<Self::Item> {
        for edge in &mut self.0 {
            if let Edge::Open(node) = edge {
                return Some(node);
            }
        }
        None
    }
}

macro_rules! axis_iterators {
    ($(#[$m:meta] $i:ident($f:path);)*) => {
        $(
            #[$m]
            #[derive(Debug)]
            pub struct $i<'a>(Option<Rc<Node<'a>>>);
            impl<'a> Clone for $i<'a> {
                fn clone(&self) -> Self {
                    $i(self.0.clone())
                }
            }

            impl<'a> Iterator for $i<'a> {
                type Item = Rc<Node<'a>>;
                fn next(&mut self) -> Option<Self::Item> {
                    let node = self.0.take();
                    self.0 = node.as_ref().map(|x| x.deref()).and_then($f);
                    node
                }
            }
        )*
    };
}

axis_iterators! {
    /// Iterator over ancestors.
    Ancestors(Node::parent);

    /// Iterator over previous siblings.
    PrevSiblings(Node::prev_sibling);

    /// Iterator over next siblings.
    NextSiblings(Node::next_sibling);

    /// Iterator over first children.
    FirstChildren(Node::first_child);

    /// Iterator over last children.
    LastChildren(Node::last_child);
}

impl<'a> Node<'a> {
    /// Returns an iterator over children.
    pub fn children(&self) -> Children<'a> {
        Children {
            front: self.first_child(),
            back: self.last_child(),
        }
    }

    /// Returns an iterator which traverses the subtree starting at this node.
    pub fn traverse(self: &Rc<Node<'a>>) -> Traverse<'a> {
        Traverse {
            root: Some(self.clone()),
            edge: None,
        }
    }

    /// Returns an iterator over this node and its descendants.
    pub fn descendants(self: &Rc<Node<'a>>) -> Descendants<'a> {
        Descendants(self.traverse())
    }

    /// Returns an iterator over ancestors.
    pub fn ancestors(self: &Rc<Node<'a>>) -> Ancestors<'a> {
        Ancestors(Some(self.clone()))
    }
}
