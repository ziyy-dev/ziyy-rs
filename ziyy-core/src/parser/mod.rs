use crate::error::Result;
use crate::splitter::fragment::{Fragment, FragmentType};
use chunk::{Chunk, ChunkData};
#[cfg(feature = "parallel")]
use rayon::prelude::*;

pub mod ansi;
pub mod chunk;
pub mod color;
pub mod tag_parser;
pub mod word_parser;

pub struct Parser {
    parse_placeholders: bool,
}

impl Default for Parser {
    fn default() -> Self {
        Self::new(true)
    }
}

impl Parser {
    pub fn new(parse_placeholders: bool) -> Self {
        Self { parse_placeholders }
    }

    pub fn parse<'a>(&self, frags: Vec<Fragment<'a>>) -> Vec<Result<Chunk<'a>>> {
        // let word_parer = WordParser::new();
        #[cfg(feature = "parallel")]
        {
            frags
                .into_par_iter()
                .map(|frag| {
                    let span = frag.span;
                    match frag.r#type {
                        FragmentType::Tag => {
                            let mut tag_parser =
                                tag_parser::TagParser::new(self.parse_placeholders);
                            Ok(Chunk {
                                data: ChunkData::Tag(tag_parser.parse(frag)?),
                                span,
                            })
                        }

                        FragmentType::Whitespace => {
                            // Handle whitespace fragments
                            Ok(Chunk {
                                data: ChunkData::WhiteSpace(frag.lexeme),
                                span,
                            })
                        }

                        FragmentType::Word => {
                            // Handle word fragments
                            // let chs = word_parer.parse(frag)?;
                            // chunks.extend_from_slice(&chs);
                            Ok(Chunk {
                                data: ChunkData::Word(frag.lexeme),
                                span,
                            })
                        }
                    }
                })
                .collect()
        }

        #[cfg(not(feature = "parallel"))]
        frags
            .into_iter()
            .map(|frag| {
                let span = frag.span;
                match frag.r#type {
                    FragmentType::Tag => {
                        let mut tag_parser = tag_parser::TagParser::new(self.parse_placeholders);
                        Ok(Chunk {
                            data: ChunkData::Tag(tag_parser.parse(frag)?),
                            span,
                        })
                    }

                    FragmentType::Whitespace => {
                        // Handle whitespace fragments
                        Ok(Chunk {
                            data: ChunkData::WhiteSpace(frag.lexeme),
                            span,
                        })
                    }

                    FragmentType::Word => {
                        // Handle word fragments
                        // let chs = word_parer.parse(frag)?;
                        // chunks.extend_from_slice(&chs);
                        Ok(Chunk {
                            data: ChunkData::Word(frag.lexeme),
                            span,
                        })
                    }
                }
            })
            .collect()
    }
}
