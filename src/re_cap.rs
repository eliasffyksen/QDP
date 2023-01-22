use std::marker::PhantomData;

use crate::parser::Parser;

pub struct ReCap<'data, 'parent, P, PT>
where
    P: Parser<'data, PT>
{
    _phantom: PhantomData<&'data PT>,
    parent: &'parent mut P,
    regex: regex::Regex,
}

impl<'data, 'parent, P, PT> ReCap<'data, 'parent, P, PT>
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

impl<'data, 'parent, P, PT> Parser<'data, Vec<String>> for ReCap<'data, 'parent, P, PT>
where
    P: Parser<'data, PT>
{
    fn parse(&mut self, data: &'data str) -> Option<(Vec<String>, &'data str)> {
        let (_, data) = self.parent.parse(data)?;

        let end = self.regex.find(data)?.end();
        let captures = self.regex.captures(data)?;

        let result = captures.iter()
            .skip(1)
            .filter(|f| f.is_some())
            .map(|m| m.unwrap().as_str().to_string())
            .collect::<Vec<_>>();

        Some((result, &data[end..]))
    }

    fn get_data(&self) -> &'data str {
        self.parent.get_data()
    }

    fn get_pos(&mut self) -> &mut usize {
        self.parent.get_pos()
    }
}