use crate::style::Style;

use super::tag::TagName;

pub struct State<'src>(pub(crate) Vec<(TagName<'src>, Style, Style)>);

impl<'src> State<'src> {
    pub fn new() -> Self {
        State(vec![])
    }

    pub fn push(&mut self, tag_name: TagName<'src>, style: Style, delta: Style) {
        self.0.push((tag_name, style, delta));
    }

    pub fn pop(&mut self) -> Option<(TagName<'src>, Style, Style)> {
        self.0.pop()
    }

    pub fn previous_tag_name(&self) -> Option<&TagName<'src>> {
        let i = self.0.len() - 1;
        self.0.get(i).map(|x| &x.0)
    }

    pub fn previous_style(&self) -> Style {
        match self.0.last() {
            Some(v) => v.1,
            None => Style::new(),
        }
    }
}
