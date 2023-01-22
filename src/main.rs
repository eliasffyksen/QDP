
mod parser;
mod str_parser;
mod find;
mod int;
mod pos_int;
mod many;
mod until;
mod map;
mod stay;
mod re_cap;

use std::mem::replace;

use parser::Parser;

const data: &str = "
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
#[derive(Debug)]
struct Monkey {
    items: Vec<i64>,
    op: Vec<String>,
    on_true: i64,
    count: u64,
    div: i64,
    on_false: i64,
}

fn main() {
    let mut p = parser::new(data);

    let mut monkeys = p.map(|p| Some(Monkey{
        count: 0,
        items: p.find("items").stay().int().many().until("\n").get()?,
        op: p.re_cap(r"old (.) (\w+)").get()?,
        div: p.find("divisible").int().get()?,
        on_true: p.find("true").int().get()?,
        on_false: p.find("false").int().get()?,
    })).many().get().unwrap();

    for _ in 0..20 {
        for i in 0..monkeys.len() {
            let monkey = &mut monkeys[i];
            let items = replace(&mut monkey.items, vec![]);
            monkey.count += items.len() as u64;

            for item in items.into_iter() {
                let monkey = &monkeys[i];
                let mut val = match monkey.op[1].as_str() {
                    "old" => item,
                    val => val.parse().unwrap(),
                };

                val = match monkey.op[0].as_str() {
                    "+" => item + val,
                    "*" => item * val,
                    _ => panic!(),
                } / 3;

                let next = match val % monkey.div {
                    0 => monkey.on_true,
                    _ => monkey.on_false
                } as usize;

                monkeys[next].items.push(val);
            }
        }
    }

    let mut res = monkeys.iter()
        .map(|m| m.count)
        .collect::<Vec<_>>();
    res.sort();
    res.reverse();

    println!("{:#?}", res[0] * res[1]);
}