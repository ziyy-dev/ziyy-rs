use std::{
    cell::RefCell,
    fmt::{self, Debug, Display, Formatter},
    rc::Rc,
};

use crate::{
    common::Span,
    parser::{
        chunk::{Chunk, ChunkData},
        tag_parser::tag::{Tag, TagType},
    },
};
pub use node::Node;

mod display;
mod iter;
mod node;

#[derive(Clone)]

pub struct Document<'a> {
    recycled: RefCell<Vec<u32>>,
    nodes: RefCell<Vec<Rc<Node<'a>>>>,
}

impl<'a> Document<'a> {
    /// Creates a new Document
    pub fn new() -> Rc<Document<'a>> {
        let doc = Rc::new(Self {
            recycled: RefCell::new(Vec::with_capacity(64)),
            nodes: RefCell::new(vec![]),
        });

        let mut tag = Tag::default();
        tag.set_name("$root".to_string());
        tag.r#type = TagType::Open;

        let node = Rc::new(Node::new(
            0,
            Chunk {
                data: ChunkData::Tag(tag),
                span: Span::inserted(),
            },
            Rc::downgrade(&Rc::clone(&doc)),
        ));

        {
            let mut nodes = doc.nodes.borrow_mut();
            nodes.push(node)
        }

        doc
    }

    pub fn get(&self, id: u32) -> Rc<Node<'a>> {
        self.nodes.borrow()[id as usize].clone()
    }

    pub fn node(&self, id: u32) -> Rc<Node<'a>> {
        self.get(id)
    }

    pub fn root(&self) -> Rc<Node<'a>> {
        self.get(0)
    }

    pub fn orphan(self: &Rc<Document<'a>>, chunk: Chunk<'a>) -> Rc<Node<'a>> {
        let doc = Rc::downgrade(&Rc::clone(self));
        let mut nodes = self.nodes.borrow_mut();
        let id = nodes.len();

        let node: Rc<Node>;

        let mut recycled = self.recycled.borrow_mut();
        if let Some(id) = recycled.pop() {
            node = Rc::new(Node::new(id, chunk, doc));
            nodes[id as usize] = node.clone();
        } else {
            node = Rc::new(Node::new(
                id.try_into().expect("maximum nodes exceeded"),
                chunk,
                doc,
            ));
            nodes.push(node.clone());
        }
        node
    }

    #[allow(clippy::len_without_is_empty)]
    pub fn len(&self) -> usize {
        self.nodes.borrow().len()
    }
}

impl<'a> Debug for Document<'a> {
    fn fmt(&self, f: &mut Formatter) -> Result<(), fmt::Error> {
        use iter::Edge;
        if f.alternate() {
            write!(f, "Document {{")?;
            for edge in self.root().traverse() {
                match edge {
                    Edge::Open(node) if node.has_children() => {
                        write!(f, " {:#?} => {{", node.chunk())?;
                    }
                    Edge::Open(node) if node.next_sibling().is_some() => {
                        write!(f, " {:#?},", node.chunk())?;
                    }
                    Edge::Open(node) => {
                        write!(f, " {:#?}", node.chunk())?;
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
            f.debug_struct("Document")
                .field("nodes", &self.nodes)
                .finish()
        }
    }
}

impl<'a> Display for Document<'a> {
    fn fmt(&self, f: &mut Formatter) -> Result<(), fmt::Error> {
        use display::Indentation;
        use iter::Edge;

        let mut indent: Indentation = Indentation::new(true);

        for edge in self.root().traverse() {
            match edge {
                Edge::Open(node) if node.has_children() => {
                    indent.indent(node.next_sibling().is_some());
                    writeln!(f, "{indent}{:#}", node.chunk().borrow())?;
                }
                Edge::Open(node) => {
                    indent.indent(node.next_sibling().is_some());
                    writeln!(f, "{indent}{:#}", node.chunk().borrow())?;
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
