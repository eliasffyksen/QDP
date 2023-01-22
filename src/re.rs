use std::marker::PhantomData;

use crate::parser::Parser;

pub struct Re<'data, 'parent, P, PT>
where
    P: Parser<'data, PT>
{
    _phantom: PhantomData<&'data PT>,
    parent: &'parent mut P,
    regex: regex::Regex,
}

impl<'data, 'parent, P, PT> Re<'data, 'parent, P, PT>
where
    P: Parser<'data, PT>
{
    pub fn new(parent: &'parent mut P, re: &str) -> Self
    {
        Self {
            _phantom: PhantomData,
            parent,
            regex: regex::Regex::new(re).unwrap(),
        }
    }
}

impl<'data, 'parent, P, PT> Parser<'data, String> for Re<'data, 'parent, P, PT>
where
    P: Parser<'data, PT>
{
    fn parse(&mut self, data: &'data str) -> Option<(String, &'data str)> {
        let (_, data) = self.parent.parse(data)?;

        let m = self.regex.find(data)?;

        Some((m.as_str().to_string(), &data[m.end()..]))
    }

    fn get_data(&self) -> &'data str {
        self.parent.get_data()
    }

    fn get_pos(&mut self) -> &mut usize {
        self.parent.get_pos()
    }
}