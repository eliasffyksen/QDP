
use crate::parser::Parser;

pub struct StrParser<'data> {
    data: &'data str,
    pos: usize,
}

impl<'data> StrParser<'data> {
    pub fn new(data: &'data str) -> StrParser<'data> {
        StrParser { data, pos: 0 }
    }
}

impl<'data> Parser<'data, ()> for StrParser<'data> {
    fn parse(&mut self, data: &'data str) -> Option<((), &'data str)> {
        Some(((), data))
    }

    fn get_data(&self) -> &'data str {
        &self.data[self.pos..]
    }

    fn get_pos(&mut self) -> &mut usize {
        &mut self.pos
    }
}
