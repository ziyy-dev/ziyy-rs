use std::{borrow::Cow, collections::HashMap, rc::Rc};

use crate::{
    Error, ErrorType, Fragment, FragmentType, Result, WordParser,
    builtin::{BUILTIN_STYLES, BUILTIN_TAGS},
    common::Span,
    parser::{
        ansi::Ansi,
        chunk::{Chunk, ChunkData},
        tag_parser::tag::{Tag, TagType},
    },
    splitter::is_whitespace,
};
use document::{Document, Node};

pub mod document;

pub struct Resolver<'a> {
    ansi_only: bool,
    tables: Vec<Rc<Node<'a>>>,
    tree: Rc<Document<'a>>,
}

impl<'a> Resolver<'a> {
    pub fn new(ansi_only: bool) -> Self {
        Self {
            ansi_only,
            tables: Vec::with_capacity(16),
            tree: Document::new(),
        }
    }

    pub fn resolve(
        &mut self,
        chunks: Vec<Result<Chunk<'a>>>,
        word_parser: &'a WordParser,
    ) -> Result<Rc<Document<'a>>> {
        let node = self.tree.root();

        let node = chunks.into_iter().fold(Ok(node), |node, chunk| {
            if let Ok(mut node) = node {
                let chunk = chunk?;
                match &chunk.data {
                    ChunkData::Tag(tag) => match tag.r#type {
                        TagType::Open => {
                            node = node.append(chunk);
                        }

                        TagType::Close => {
                            {
                                let this = node.chunk().borrow();
                                let open_name = this.tag().unwrap().name();
                                if open_name != tag.name() {
                                    return Err(Error::new(
                                        ErrorType::InvalidTag,
                                        format!(
                                            "Mismatched tags: <{}>...</{}>",
                                            open_name,
                                            tag.name()
                                        ),
                                        chunk.span,
                                    ));
                                }
                            }

                            node.append(chunk);
                            node = node.parent().unwrap();
                        }

                        TagType::SelfClose => {
                            node.append(chunk);
                        }
                    },
                    ChunkData::WhiteSpace(_) => {
                        node.append(chunk);
                    }
                    ChunkData::Word(_) => {
                        node.append(chunk);
                    }
                }
                Ok(node)
            } else {
                node
            }
        })?;

        if self.ansi_only {
            Resolver::optimize_ansi(&node);
            return Ok(self.tree.clone());
        }

        {
            let mut resolved = Vec::with_capacity(128);
            self.parse_words(&node, &word_parser, &mut resolved)?;
            for (node, chunks) in resolved {
                for chunk in chunks {
                    node.insert_before(chunk?);
                }
                node.detach(true);
            }
        }

        let mut detachables = Vec::with_capacity(128);
        {
            let mut bindings: HashMap<String, Tag> = HashMap::new();
            Resolver::resolve_bindings(&mut bindings, &node, &mut detachables);
            for node in detachables.drain(..) {
                node.detach(true);
            }
        }

        {
            Resolver::optimize_ws(&node, &mut detachables);
            for node in &detachables {
                node.detach(true);
            }
        }

        Resolver::_resolve(&node, "$root");
        Resolver::optimize_styles(&node);
        Resolver::optimize_ansi(&node);
        self.set_tables();

        Ok(self.tree.clone())
    }

    fn parse_words(
        &mut self,
        node: &Rc<Node<'a>>,
        word_parser: &'a WordParser,
        resolved: &mut Vec<(Rc<Node<'a>>, Vec<Result<Chunk<'a>>>)>,
    ) -> crate::Result<()> {
        for child in node.children() {
            let child_chunk = child.chunk().borrow_mut();
            if child_chunk.is_word() {
                let word = child_chunk.word().unwrap();
                let chs = word_parser.parse(Fragment {
                    r#type: FragmentType::Word,
                    lexeme: word.clone(),
                    span: child_chunk.span,
                });
                resolved.push((child.clone(), chs));
            } else if child_chunk.is_tag() {
                let tag = child_chunk.tag().unwrap();
                if tag.r#type == TagType::Open {
                    let name = tag.name();
                    if matches!(name.as_str(), "pre" | "a" | "script" | "style") {
                        continue;
                    } else if name == "table" {
                        self.tables.push(child.clone());
                    } else if name == "td" {
                        child.insert_before(Chunk {
                            data: ChunkData::WhiteSpace(Cow::Borrowed("")),
                            span: Span::inserted(),
                        });
                    }
                }

                self.parse_words(&child, word_parser, resolved)?;
                continue;
            }
        }

        Ok(())
    }

    fn set_tables(&self) {
        for table in &self.tables {
            let indent = table
                .chunk()
                .borrow_mut()
                .tag_mut()
                .unwrap()
                .custom()
                .parse()
                .unwrap_or(0);
            let indent = " ".repeat(indent);

            let mut widths = Vec::with_capacity(16);
            let mut _table: Vec<Vec<(Rc<Node>, usize)>> = Vec::with_capacity(16);

            let mut x = 0;
            for tr in table.children() {
                if tr.chunk().borrow().is_ws() {
                    continue;
                }

                _table.push(Vec::with_capacity(16));

                let mut y = 0;
                for td in tr.children() {
                    if td.chunk().borrow().is_ws() {
                        td.chunk().borrow_mut().data = ChunkData::WhiteSpace(Cow::Borrowed(" "));
                        continue;
                    }
                    let mut len = 0;
                    td.word_len(&mut len);
                    if let Some(prev_len) = widths.get(y) {
                        if len > *prev_len {
                            widths[y] = len;
                        }
                    } else {
                        widths.push(len);
                    }
                    _table[x].push((td.clone(), len));

                    y += 1
                }

                if let Some(first) = tr.first_child() {
                    first.insert_before(Chunk {
                        data: ChunkData::WhiteSpace(Cow::Owned(indent.clone())),
                        span: Span::inserted(),
                    });
                }

                x += 1
            }

            for row in _table {
                for (i, (col, width)) in row.iter().enumerate() {
                    let lwidth = &widths[i];
                    if lwidth == width {
                        continue;
                    }

                    let indent = " ".repeat(lwidth - width);
                    if let Some(last) = col.last_child() {
                        last.insert_after(Chunk {
                            data: ChunkData::WhiteSpace(Cow::Owned(indent.clone())),
                            span: Span::inserted(),
                        });
                    }
                }
            }
        }
        // self.tables.clear();
    }

    /// Resolve all declared bindings: <let />
    fn resolve_bindings(
        bindings: &mut HashMap<String, Tag>,
        node: &Rc<Node<'a>>,
        detachables: &mut Vec<Rc<Node<'a>>>,
    ) {
        for child in node.children() {
            let mut child_chunk = child.chunk().borrow_mut();
            if child_chunk.is_tag() {
                let tag = child_chunk.tag_mut().unwrap();
                let name = tag.name().clone();

                if !BUILTIN_TAGS.contains(&name.as_str()) {
                    for ansector in child.ancestors() {
                        if let Some(binding) = bindings.get(&format!("{}/{}", ansector.id(), name))
                        {
                            tag.inherit(binding);
                            break;
                        }
                    }
                }

                if !tag.class().is_empty() {
                    for class in tag
                        .class()
                        .clone()
                        .split(|x| is_whitespace(x as u8))
                        .filter(|s| !s.is_empty())
                        .rev()
                    {
                        for ansector in child.ancestors() {
                            if let Some(binding) = BUILTIN_STYLES.get(class) {
                                tag.inherit(binding);
                                break;
                            }
                            if let Some(binding) =
                                bindings.get(&format!("{}/{}", ansector.id(), class))
                            {
                                tag.inherit(binding);
                                break;
                            }
                        }
                    }
                }

                if name == "let" {
                    let tag = tag.clone();
                    let name = tag.custom();
                    if !BUILTIN_TAGS.contains(&name.as_str()) {
                        let id = node.id();
                        bindings.insert(format!("{id}/{name}"), tag);
                        detachables.push(child.clone());
                    }
                }
            }
            Resolver::resolve_bindings(bindings, &child, detachables);
        }
    }

    /// Optimizes Excess Whitespace
    fn optimize_ws(node: &Rc<Node<'a>>, detachables: &mut Vec<Rc<Node<'a>>>) {
        for child in node.children() {
            let mut child_chunk = child.chunk().borrow_mut();
            if child_chunk.is_ws() {
                if child.id() == 1 {
                    detachables.push(child.clone());
                } else if child.id() as usize == child.doc().len() - 1
                    && child_chunk.ws().is_some_and(|s| s.contains("\n"))
                {
                    child_chunk.data = ChunkData::WhiteSpace(Cow::Borrowed("\n"));
                } else {
                    child_chunk.data = ChunkData::WhiteSpace(Cow::Borrowed(" "));
                }

                if let Some(first) = child.next_sibling().and_then(|next| next.first_child()) {
                    if first.chunk().borrow().is_ws() {
                        detachables.push(first);
                    }
                } else if child.next_sibling().is_some_and(|node| {
                    node.chunk()
                        .borrow()
                        .is_tag_and(|tag| tag.r#type == TagType::Close)
                }) {
                    if let Some(next) = node.next_sibling() {
                        if next.chunk().borrow().is_ws() {
                            if child.next_sibling().is_some_and(|node| {
                                node.chunk().borrow().is_tag_and(|tag| tag.name() == "td")
                            }) {
                                detachables.push(child.clone());
                            } else {
                                detachables.push(next);
                            }
                        }
                    }
                } else if let Some(next) = child.next_sibling() {
                    if next.chunk().borrow().is_ws() {
                        detachables.push(next);
                    }
                }
            } else if child_chunk.is_tag() {
                let name = child_chunk.tag().unwrap().name();
                if matches!(name.as_str(), "p" | "ziyy" | "$root" | "div") {
                    if let Some(first) = child.first_child() {
                        if first.chunk().borrow().is_ws() {
                            detachables.push(first);
                        }
                    }
                } else if name == "br" {
                    if let Some(prev) = child.prev_sibling() {
                        if prev.chunk().borrow().is_ws() {
                            detachables.push(prev);
                        }
                    }

                    if let Some(next) = child.next_sibling() {
                        if next.chunk().borrow().is_ws() {
                            detachables.push(next);
                        }
                    }
                } else if matches!(name.as_str(), "pre") {
                    continue;
                }
            }

            Resolver::optimize_ws(&child, detachables);
        }
    }

    fn _resolve(node: &Rc<Node>, node_name: &str) {
        for child in node.children() {
            let mut child_chunk = child.chunk().borrow_mut();
            if child_chunk.is_tag() {
                let tag = child_chunk.tag_mut().unwrap();
                if tag.r#type == TagType::Open {
                    let name = tag.name();
                    if matches!(name.as_str(), "ziyy" | "p" | "div" | "pre" | "table" | "tr") {
                        if matches!(
                            node_name,
                            "ziyy" | "$root" | "p" | "div" | "pre" | "table" | "tr"
                        ) && node
                            .first_child()
                            .is_some_and(|first| first.id() == child.id())
                        {
                        } else {
                            child.insert_before(Chunk {
                                data: ChunkData::WhiteSpace(Cow::Borrowed("\n")),
                                span: Span::inserted(),
                            });
                        }
                    } else if name == "a" {
                        for grand_child in child.children() {
                            grand_child.strip_styles();
                        }
                    }

                    if let Some(last) = child.last_child() {
                        let last_chunk = last.chunk().borrow_mut();
                        if !last_chunk.is_tag_and(|tag| tag.r#type == TagType::Close) {
                            last.insert_after(Chunk {
                                data: ChunkData::Tag(tag.close()),
                                span: Span::inserted(),
                            });
                        }
                    }
                }

                Resolver::_resolve(&child, tag.name());
            } else {
                Resolver::_resolve(&child, node_name);
            }
        }
    }

    /// Optimize styles
    fn optimize_styles(node: &Rc<Node>) {
        let mut stack: Vec<(String, Ansi, Ansi)> = Vec::with_capacity(1024);
        for child in node.descendants() {
            let mut child_chunk = child.chunk().borrow_mut();
            if child_chunk.is_tag() {
                let tag = child_chunk.tag_mut().unwrap();
                match tag.r#type {
                    TagType::Open => {
                        let (_, prev_style, _) = stack.last().cloned().unwrap_or_default();
                        //
                        let new_style = prev_style.clone() + tag.clone().ansi;
                        let new_delta = tag.clone().ansi - prev_style.clone();

                        tag.ansi = new_delta.clone();

                        stack.push((tag.name().clone(), new_style, new_delta));
                    }

                    TagType::Close => {
                        let mut current_tag = stack.pop().unwrap_or_default();
                        let new_tag = current_tag.clone();
                        while current_tag.0 == "$ansi" {
                            current_tag = stack.pop().unwrap_or_default();
                        }

                        //
                        tag.ansi = !new_tag.2.clone();

                        if let Some((_, prev, _)) = stack.last() {
                            if !prev.fg_color().is_empty() && !tag.fg_color().is_empty() {
                                tag.ansi.set_fg_color(prev.fg_color().clone());
                            }
                            if !prev.bg_color().is_empty() && !tag.bg_color().is_empty() {
                                tag.ansi.set_bg_color(prev.bg_color().clone());
                            }
                        }
                    }

                    TagType::SelfClose => {}
                }
            }
        }
    }

    fn optimize_ansi(node: &Rc<Node>) {
        let decendants: Vec<_> = node.descendants().collect();

        let mut i = 0;
        while i < decendants.len() {
            let first = &decendants[i];
            let mut first_chunk = first.chunk().borrow_mut();
            if first_chunk.is_tag() {
                if let Some(second) = decendants.get(i + 1) {
                    let mut second_chunk = second.chunk().borrow_mut();
                    if second_chunk.is_tag() {
                        let first_tag = first_chunk.tag_mut().unwrap();
                        let second_tag = second_chunk.tag_mut().unwrap();

                        *second_tag = first_tag.clone() + second_tag.clone();
                        first_tag.reset_styles();
                    }
                }
            }
            i += 1;
        }
    }
}
