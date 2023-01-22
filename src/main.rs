
mod parser;
mod str_parser;
mod find;
mod int;
mod many;
mod until;
mod map;
mod stay;

use parser::Parser;

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

    let mut p = parser::new(data);

    #[derive(Debug)]
    struct Monkey {
        items: Vec<i64>,
        on_true: i64,
        div: i64,
        on_false: i64,
    }

    let monkeys = p.map(|p| {
        Some(Monkey{
            items: p.find("items").stay().int().many().until("\n").get()?,
            div: p.find("divisible").int().get()?,
            on_true: p.find("true").int().get()?,
            on_false: p.find("false").int().get()?,
        })
    }).many().get().unwrap();

    println!("{:?}", monkeys);
}