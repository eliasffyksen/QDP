use std::marker::PhantomData;

use crate::parser::Parser;

pub struct Until<'data, 'parent, 'pat, P, PT>
where
    P: Parser<'data, PT>
{
    _phantom: PhantomData<&'data PT>,
    parent: &'parent mut P,
    pat: &'pat str
}

impl<'data, 'parent, 'pat, P, PT> Until<'data, 'parent, 'pat, P, PT>
where
  P: Parser<'data, PT>
{
    pub fn new(parent: &'parent mut P, pat: &'pat str) -> Self {
        Self { _phantom: PhantomData, parent, pat }
    }
}

impl<'data, 'parent, 'pat, P, PT> Parser<'data, PT> for Until<'data, 'parent, 'pat, P, PT>
where
    P: Parser<'data, PT>
{
    fn parse(&mut self, data: &'data str) -> Option<(PT, &'data str)> {
      let mut stop = data.len();

      if let Some(pos) = data.find(self.pat) {
        stop = pos + self.pat.len();
      }

      let (val, _) = self.parent.parse(&data[..stop])?;

      Some((val, &data[stop..]))
    }

    fn get_data(&self) -> &'data str {
      self.parent.get_data()
    }

    fn get_pos(&mut self) -> &mut usize {
        self.parent.get_pos()
    }
}
