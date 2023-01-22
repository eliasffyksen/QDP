use std::marker::PhantomData;

use crate::parser::Parser;

pub struct Many<'data, 'parent, P, PT>
where
    P: Parser<'data, PT>,
{
    _phantom: PhantomData<&'data PT>,
    parent: &'parent mut P,
}

impl<'data, 'parent, P, PT> Many<'data, 'parent, P, PT>
where
    P: Parser<'data, PT>,
{
    pub fn new(parent: &'parent mut P) -> Many<'data, 'parent, P, PT> {
        Many { _phantom: PhantomData, parent }
    }
}

impl<'data, 'parent, P, PT> Parser<'data, Vec<PT>> for Many<'data, 'parent, P, PT>
where
    P: Parser<'data, PT>,
{
    fn parse(&mut self, data: &'data str) -> Option<(Vec<PT>, &'data str)> {

        if data.len() == 0 {
            return None;
        }

        let mut data = data;
        let mut result = vec![];

        while let Some((val, new_data)) = self.parent.parse(data) {
            result.push(val);
            data = new_data;
        }

        Some((result, data))
    }

    fn get_data(&self) -> &'data str {
        self.parent.get_data()
    }

    fn get_pos(&mut self) -> &mut usize {
        self.parent.get_pos()
    }
}
