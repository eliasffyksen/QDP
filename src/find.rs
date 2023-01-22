
use std::marker::PhantomData;

use crate::parser::Parser;

pub struct Find<'data, 'parent, 'pat, P, PT>
where
    P: Parser<'data, PT>
{
    _phantom: PhantomData<&'data PT>,
    parent: &'parent mut P,
    pat: &'pat str,
}

impl<'data, 'parent, 'pat, P, PT> Find<'data, 'parent, 'pat, P, PT>
where
    P: Parser<'data, PT>
{
    pub fn new(parent: &'parent mut P, pat: &'pat str)
      -> Find<'data, 'parent, 'pat, P, PT>
    {
        Self { _phantom: PhantomData, parent, pat }
    }
}

impl<'data, 'parent, 'pat, P, PT> Parser<'data, &'pat str>
    for Find<'data, 'parent, 'pat, P, PT>
where
    P: Parser<'data, PT>
{
    fn parse(&mut self, data: &'data str) -> Option<(&'pat str, &'data str)> {
        let (_, data) = self.parent.parse(data)?;

        let pos = data.find(&self.pat)?;

        Some((&self.pat, &data[pos + self.pat.len()..]))
    }

    fn get_data(&self) -> &'data str {
        self.parent.get_data()
    }

    fn get_pos(&mut self) -> &mut usize {
        self.parent.get_pos()
    }
}
