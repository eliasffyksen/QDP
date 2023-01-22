use std::{marker::PhantomData};

use crate::{parser::Parser, str_parser::StrParser};

pub struct Map<'data, 'parent, P, PT, F>
where
    P: Parser<'data, PT>
{
    _phantom: PhantomData<&'data PT>,
    parent: &'parent mut P,
    f: F,
}

impl<'data, 'parent, P, PT, F> Map<'data, 'parent, P, PT, F>
where
  P: Parser<'data, PT>
{
    pub fn new(parent: &'parent mut P, f: F) -> Self {
        Self { _phantom: PhantomData, parent, f }
    }
}

impl<'data, 'parent, P, PT, F, T>
    Parser<'data, T>
    for Map<'data, 'parent, P, PT, F>
where
    F: FnMut(&mut StrParser) -> Option<T>,
    P: Parser<'data, PT>,
{
    fn parse(&mut self, data: &'data str) -> Option<(T, &'data str)> {
        let (_, data) = self.parent.parse(data)?;

        let mut parser = StrParser::new(data);
        let result = (self.f)(&mut parser)?;

        Some((result, parser.get_data()))
    }

    fn get_data(&self) -> &'data str {
        self.parent.get_data()
    }

    fn get_pos(&mut self) -> &mut usize {
        self.parent.get_pos()
    }
}
