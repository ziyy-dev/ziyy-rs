use crate::style::Style;

use super::tag::TagName;

pub struct State(Vec<(TagName, Style, Style)>);

impl State {
    pub fn new() -> State {
        State(vec![(TagName::Ziyy, Style::new(), Style::new())])
    }

    #[cfg(test)]
    pub fn is_empty(&self) -> bool {
        self.0.len() == 1
    }

    pub fn push(&mut self, tag_name: TagName, style: Style, delta: Style) {
        let l = self.0.len() - 1;
        let mut pstyle = self.0.get(l).unwrap().clone().1;
        // println!("{style:?}");
        pstyle.add(style);
        self.0.push((tag_name, pstyle, delta));
    }

    pub fn pop(&mut self) -> Option<(TagName, Style, Style)> {
        self.0.pop()
    }

    pub fn current_tag_name(&self) -> Option<&TagName> {
        let i = self.0.len() - 1;
        self.0.get(i).map(|x| &x.0)
    }

    pub fn current_style(&self) -> Option<&Style> {
        let i = self.0.len() - 1;
        self.0.get(i).map(|x| &x.1)
    }

    pub fn previous_style(&self) -> Option<&Style> {
        let i = self.0.len() - 1;
        self.0.get(i).map(|x| &x.1)
    }
}
