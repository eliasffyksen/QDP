use crate::{str_parser::StrParser, parser::Parser};

pub struct Stay<'data>
where
{
    tmp_parser: StrParser<'data>,
}

impl<'data> Stay<'data>
{
    pub fn new<P, PT>(parent: &mut P) -> Self
    where
        P: Parser<'data, PT>
    {
        parent.get();

        Self {
            tmp_parser: StrParser::new(parent.get_data())
        }
    }
}

impl<'data, 'parent> Parser<'data, ()> for Stay<'data>
{
    fn parse(&mut self, data: &'data str) -> Option<((), &'data str)> {
        Some(((), data))
    }

    fn get_data(&self) -> &'data str {
        self.tmp_parser.get_data()
    }

    fn get_pos(&mut self) -> &mut usize {
        self.tmp_parser.get_pos()
    }
}
