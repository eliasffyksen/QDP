
use crate::{str_parser::StrParser, find::Find, int::Int, many::Many, until::Until, map::Map, stay::Stay, re_cap::ReCap, pos_int::PosInt, count::Count, re::Re};

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

    fn count<'pat>(&mut self, pat: &'pat str) -> Count<'data, '_, 'pat, Self, T>
    where
        Self: Sized,
    {
      Count::new(self, pat)
    }

    fn int<'parent>(&'parent mut self) -> Int<'data, 'parent, Self, T>
    where
        Self: Sized,
    {
      Int::new(self)
    }

    fn pos_int<'parent>(&'parent mut self) -> PosInt<'data, 'parent, Self, T>
    where
        Self: Sized,
    {
      PosInt::new(self)
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

    fn re_cap(&mut self, re: &str) -> ReCap<'data, '_, Self, T>
    where
        Self: Sized,
    {
        ReCap::new(self, re)
    }

    fn re(&mut self, re: &str) -> Re<'data, '_, Self, T>
    where
        Self: Sized,
    {
        Re::new(self, re)
    }
}

pub fn new<'a>(data: &'a str) -> StrParser<'a> {
  StrParser::new(data)
}
