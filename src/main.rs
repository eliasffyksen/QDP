
mod parser;
mod str_parser;
mod find;
mod count;
mod int;
mod pos_int;
mod many;
mod until;
mod map;
mod stay;
mod re_cap;
mod re;

use parser::Parser;

const data: &str = "
    [D]    
[N] [C]    
[Z] [M] [P]
 1   2   3 

move 1 from 2 to 1
move 3 from 1 to 3
move 2 from 2 to 1
move 1 from 1 to 2
";
fn main() {
    let mut p = parser::new(data);

    let pile_count = p.stay().find("1").stay().count("  ").until("\n").get().unwrap() + 1;
    let mut piles = vec![];
    for _ in 0..pile_count {
        piles.push(vec![])
    }

    for v in p.re(r"    |\[\w\] ").many().until("\n")
        .many().until("1").get().unwrap() {
        for (i, v) in v.into_iter().enumerate() {
            match v.as_str() {
                "    " => (),
                _ => piles[i].push(v),
            }
        }
    }

    p.find("move").get();

    let actions = p.int().many().until("\n").many().get();

    println!("{:#?}", piles);
    println!("{:#?}", actions);
}