
use crate::{str_parser::StrParser, find::Find, int::Int, many::Many, until::Until, map::Map, stay::Stay};

pub trait Parser<'data, T> {
    fn parse(&mut self, data: &'data str) -> Option<(T, &'data str)>;
    fn get_data(&self) -> &'data str;
    fn get_pos(&mut self) -> &mut usize;

    fn get(&mut self) -> Option<T> {
        let data = self.get_data();
        let orig_len = data.len();

        let (result, data) = self.parse(data)?;
        let new_len = data.len();

        *self.get_pos() += orig_len - new_len;
        Some(result)
    }

    fn find<'pat>(&mut self, pat: &'pat str) -> Find<'data, '_, 'pat, Self, T>
    where
        Self: Sized,
    {
      Find::new(self, pat)
    }

    fn int<'parent>(&'parent mut self) -> Int<'data, 'parent, Self, T>
    where
        Self: Sized,
    {
      Int::new(self)
    }

    fn many<'parent>(&'parent mut self) -> Many<'data, 'parent, Self, T>
    where
        Self: Sized,
    {
      Many::new(self)
    }

    fn until<'pat>(&mut self, pat: &'pat str) -> Until<'data, '_, 'pat, Self, T>
    where
        Self: Sized,
    {
      Until::new(self, pat)
    }

    fn map<F, NT>(&mut self, f: F) -> Map<'data, '_, Self, T, F>
    where
        Self: Sized,
        F: FnMut(&mut StrParser) -> Option<NT>,
    {
      Map::new(self, f)
    }

    fn stay(&mut self) -> Stay<'data>
    where
      Self: Sized,
    {
      Stay::new(self)
    }
}

pub fn new<'a>(data: &'a str) -> StrParser<'a> {
  StrParser::new(data)
}
