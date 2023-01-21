use std::marker::PhantomData;
use lazy_static::lazy_static;
use regex::Regex;

trait Parser<'a, T> {
    fn get(&mut self) -> Option<T>;
    fn get_str(&self) -> &'a str;
    fn advance(&mut self, bytes: i64);

    fn find<'b, 'c>(&'b mut self, pat: &'c str) -> FindParser<'a, 'b, 'c, Self, T>
    where
        Self: Sized
    {
        FindParser {
            phantom: PhantomData,
            parent: self,
            pat: pat,
        }
    }

    fn stay<'b>(&'b mut self) -> StayParser<'a, 'b, Self, T>
    where
        Self: Sized
    {
        StayParser {
            phantom: PhantomData,
            parent: self,
            advanced_count: 0,
        }
    }

    fn int<'b>(&'b mut self) -> IntParser<'a, 'b, Self, T>
    where
        Self: Sized
    {
        IntParser {
            phantom: PhantomData,
            parent: self,
        }
    }

    fn map<'b, 'c, F, NT>(&'b mut self, f: F) -> LoopParser<'a, 'b, Self, T, F>
    where
        Self: Sized,
        F: FnMut(&mut StrParser) -> Option<NT>,
    {
        LoopParser {
            phantom: PhantomData,
            parent: self,
            f,
        }
    }

    fn until<'b, 'c>(&'b mut self, pat: &'c str) -> UntilParser<'a, 'b, 'c, Self, T>
    where
        Self: Sized
    {
        UntilParser {
            phantom: PhantomData,
            parent: self,
            pat,
            to: None
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

struct FindParser<'a, 'b, 'c, P, PT>
where
    P: Parser<'a, PT>
{
    phantom: PhantomData<&'a PT>,
    parent: &'b mut P,
    pat: &'c str,
}

impl<'a, 'b, 'c, P, PT> Parser<'a, &'c str> for FindParser<'a, 'b, 'c, P, PT>
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

struct StayParser<'a, 'b, P, T>
where
    P: Parser<'a, T>
{
    phantom: PhantomData<&'a T>,
    parent: &'b mut P,
    advanced_count: i64,
}

impl<'a, 'b, P, T> Parser<'a, ()> for StayParser<'a, 'b, P, T>
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

impl<'a, 'b, P, T> Drop for StayParser<'a, 'b, P, T>
where
    P: Parser<'a, T>
{
    fn drop(&mut self) {
        self.parent.advance(-self.advanced_count);
    }
}

struct IntParser<'a, 'b, P, PT>
where
    P: Parser<'a, PT>
{
    phantom: PhantomData<&'a PT>,
    parent: &'b mut P
}

impl<'a, 'b, P, PT> Parser<'a, i64> for IntParser<'a, 'b, P, PT>
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

struct LoopParser<'a, 'b, P, PT, F>
where
    P: Parser<'a, PT>
{
    phantom: PhantomData<&'a PT>,
    parent: &'b mut P,
    f: F,
}

impl<'a, 'b, 'c, P, PT, F, T>
    Parser<'a, Vec<T>>
    for LoopParser<'a, 'b, P, PT, F>
where
    F: FnMut(&mut StrParser) -> Option<T>,
    P: Parser<'a, PT>,
{
    fn get(&mut self) -> Option<Vec<T>> {
        self.parent.get()?;

        let mut parser = StrParser::new(self.get_str());

        let mut result = vec![];

        while let Some(x) = (self.f)(&mut parser) {
            result.push(x)
        }

        Some(result)
    }

    fn get_str(&self) -> &'a str {
        self.parent.get_str()
    }

    fn advance(&mut self, bytes: i64) {
        self.parent.advance(bytes)
    }
}

struct UntilParser<'a, 'b, 'c, P, PT>
where
    P: Parser<'a, PT>
{
    phantom: PhantomData<&'a PT>,
    parent: &'b mut P,
    pat: &'c str,
    to: Option<usize>,
}

impl<'a, 'b, 'c, P, PT> Parser<'a, ()> for UntilParser<'a, 'b, 'c, P, PT>
where
    P: Parser<'a, PT>
{
    fn get(&mut self) -> Option<()> {
        self.parent.get()?;

        if let Some(usize) = self.parent.get_str().find(self.pat) {
            self.to = Some(usize)
        }

        Some(())
    }

    fn get_str(&self) -> &'a str {
        if let Some(to) = self.to {
            &self.parent.get_str()[..to]
        } else {
            &self.parent.get_str()
        }
    }

    fn advance(&mut self, bytes: i64) {
        self.parent.advance(bytes)
    }
}

#[derive(Debug)]
struct Monkey {
    items: Vec<i64>,
    on_true: i64,
    div: i64,
    on_false: i64,
}

fn main() {
    let data = "
Monkey 0:
  Starting items: 79, 98
  Operation: new = old * 19
  Test: divisible by 23
    If true: throw to monkey 2
    If false: throw to monkey 3

Monkey 1:
  Starting items: 54, 65, 75, 74
  Operation: new = old + 6
  Test: divisible by 19
    If true: throw to monkey 2
    If false: throw to monkey 0

Monkey 2:
  Starting items: 79, 60, 97
  Operation: new = old * old
  Test: divisible by 13
    If true: throw to monkey 1
    If false: throw to monkey 3

Monkey 3:
  Starting items: 74
  Operation: new = old + 3
  Test: divisible by 17
    If true: throw to monkey 0
    If false: throw to monkey 1
";

    let mut p = StrParser::new(data);

    let monkeys = p.map(|p| Some(Monkey{
        items: p.find("Starting items").until("\n").map(|p|
            p.int().get()).get()?,
        div: p.find("divisible").int().get()?,
        on_true: p.find("true").int().get()?,
        on_false: p.find("false").int().get()?,
    })).get().unwrap();

    println!("{:?}", monkeys);
}