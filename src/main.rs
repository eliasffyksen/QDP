use std::marker::PhantomData;
use lazy_static::lazy_static;
use regex::Regex;

trait Parser<'a, T> {
    fn get(&mut self) -> Option<T>;
    fn get_str(&self) -> &'a str;
    fn advance(&mut self, bytes: i64);

    fn find<'b, 'c, P>(&'b mut self, pat: &'c str) -> FindParser<'b, 'c, P>
    where
        Self: Sized
    {
        FindParser {
            phantom: PhantomData,
            parent: self,
            pat: pat,
        }
    }

    fn stay<'b, P>(&'b mut self) -> StayParser<'b, P>
    where
        Self: Sized
    {
        StayParser {
            phantom: PhantomData,
            parent: self,
            advanced_count: 0,
        }
    }

    fn int<'b, P>(&'b mut self) -> IntParser<'a, P>
    where
        Self: Sized
    {
        IntParser {
            phantom: PhantomData,
            parent: self,
        }
    }
}

struct StrParser<'a> {
    str: &'a str,
    pos: usize,
}

impl StrParser<'_> {
    fn new<'a>(input: &'a str) -> StrParser<'a> {
        StrParser { str: input, pos: 0 }
    }
}

impl<'a> Parser<'a, ()> for StrParser<'a> {
    fn get(&mut self) -> Option<()> {
        Some(())
    }

    fn get_str(&self) -> &'a str {
        return &self.str[self.pos..]
    }

    fn advance(&mut self, bytes: i64) {
        let res = bytes + self.pos as i64;
        self.pos = res as usize;
    }
}

struct FindParser<'a, 'b, P>
{
    parent: &'a P,
    pat: &'b str,
}

impl<'a, 'b, 'c, PT, P> Parser<'a, &'c str> for FindParser<'b, 'c, P>
where
    P: Parser<'a, PT>
{
    fn get(&mut self) -> Option<&'c str> {
        self.parent.get()?;
        let data = self.get_str();
        let pos = data.find(&self.pat)?;
        self.advance((pos + self.pat.len()) as i64);
        Some(self.pat)
    }

    fn get_str(&self) -> &'a str {
        self.parent.get_str()
    }

    fn advance(&mut self, bytes: i64) {
        self.parent.advance(bytes)
    }
}

struct StayParser<'a, P>
{
    parent: &'a mut P,
    advanced_count: i64,
}

impl<'a, 'b, P, T> Parser<'a, ()> for StayParser<'b, P>
where
    P: Parser<'a, T>
{
    fn get(&mut self) -> Option<()> {
        self.parent.get()?;
        Some(())
    }

    fn get_str(&self) -> &'a str {
        self.parent.get_str()
    }

    fn advance(&mut self, bytes: i64) {
        self.advanced_count += bytes;
        self.parent.advance(bytes)
    }
}

impl<'a, 'b, P, T> Drop for StayParser<'b, P>
where
    P: Parser<'a, T>
{
    fn drop(&mut self) {
        self.parent.advance(-self.advanced_count);
    }
}

struct IntParser<'a, P>
{
    parent: &'a mut P
}

impl<'a, 'b, P, PT> Parser<'a, i64> for IntParser<'b, P>
where
    P: Parser<'a, PT>
{
    fn get(&mut self) -> Option<i64> {
        lazy_static! {
            static ref INT_RE: regex::Regex = Regex::new(r"-?\d+").unwrap();
        }

        self.parent.get()?;

        let m = INT_RE.find(self.get_str())?;
        let start = m.start();
        let end = m.end();

        let result = &self.get_str()[start..end];
        let result = result.parse::<i64>().expect("too large number");
        self.advance(end as i64);
        Some(result)
    }

    fn get_str(&self) -> &'a str {
        self.parent.get_str()
    }

    fn advance(&mut self, bytes: i64) {
        self.parent.advance(bytes)
    }
}

// struct LoopParser<'a, P, PT, F> {
//     parent: &'a 
// }

fn main() {
    let data = "
    This is some 666 data to parse.
    ";
    let mut p = StrParser::new(data);

    println!("{:?}", p.get_str());
    println!("{:?}", p.find("is").stay().find("data").get());

    println!("{:?}", p.get_str());
    println!("{:?}", p.find("data").get());
}
