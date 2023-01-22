use std::marker::PhantomData;

use lazy_static::lazy_static;
use regex::Regex;

use crate::parser::Parser;

pub struct PosInt<'data, 'parent, P, PT>
where
    P: Parser<'data, PT>
{
    _phantom: PhantomData<&'data PT>,
    parent: &'parent mut P
}

impl<'data, 'parent, P, PT> PosInt<'data, 'parent, P, PT>
where
  P: Parser<'data, PT>
{
    pub fn new(parent: &'parent mut P) -> Self {
        Self { _phantom: PhantomData, parent }
    }
}

impl<'data, 'parent, P, PT> Parser<'data, i64> for PosInt<'data, 'parent, P, PT>
where
    P: Parser<'data, PT>
{

    fn parse(&mut self, data: &'data str) -> Option<(i64, &'data str)> {
        let (_, data) = self.parent.parse(data)?;

        lazy_static! {
            static ref INT_RE: regex::Regex = Regex::new(r"\d+").unwrap();
        }

        let m = INT_RE.find(data)?;
        let start = m.start();
        let end = m.end();

        let result = &data[start..end];
        let result = result.parse::<i64>().expect("too large number");

        Some((result, &data[end..]))
    }

    fn get_data(&self) -> &'data str {
        self.parent.get_data()
    }

    fn get_pos(&mut self) -> &mut usize {
        self.parent.get_pos()
    }
}
