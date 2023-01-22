use std::marker::PhantomData;

use crate::parser::Parser;

pub struct Count<'data, 'parent, 'pat, P, PT>
where
    P: Parser<'data, PT>
{
    _phantom: PhantomData<&'data PT>,
    parent: &'parent mut P,
    pat: &'pat str,
}

impl<'data, 'parent, 'pat, P, PT> Count<'data, 'parent, 'pat, P, PT>
where
    P: Parser<'data, PT>
{
    pub fn new(parent: &'parent mut P, pat: &'pat str)
      -> Count<'data, 'parent, 'pat, P, PT>
    {
        Self { _phantom: PhantomData, parent, pat }
    }
}

impl<'data, 'parent, 'pat, P, PT> Parser<'data, usize>
    for Count<'data, 'parent, 'pat, P, PT>
where
    P: Parser<'data, PT>
{
    fn parse(&mut self, data: &'data str) -> Option<(usize, &'data str)> {
        let (_, mut data) = self.parent.parse(data)?;
        let mut count = 0;

        while let Some(pos) = data.find(&self.pat) {
          count += 1;
          data = &data[pos + self.pat.len()..]
        }

        Some((count, data))
    }

    fn get_data(&self) -> &'data str {
        self.parent.get_data()
    }

    fn get_pos(&mut self) -> &mut usize {
        self.parent.get_pos()
    }
}
