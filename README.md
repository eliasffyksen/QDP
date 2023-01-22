# Rust Quick and Dirty Parser

This is a work in progress library written for quick and dirty
parsing in rust with little input validation. The aim of the project
is to aid advent of code style programming competitions in rust.

The library works by chaining parsers similarly to how rust Iterators work.

Once the get is called parser will either succeed and advance the input as a
whole or return `None` without advancing at all. It is all or nothing.

The parser will skip input indescriminatly if it can parse later input.

The library is heavly based on borrowing so no data is copied or moved during
the parsing and no heap is used exept for storing the outvalues in `Vector`
and `String` results.

## Examples

Here are some examples from Advent of Code 2022.

All examples assume parser:
```rust
let mut p = parser::new(input);
```

### Advent of Code 2022 day 1

Input:
```
1000
2000
3000

4000

5000
6000

7000
8000
9000

10000
```

Code:
```rust
p.int().many().until("\n\n").many().get()
```

Result `Option<Vec<Vec<i64>>>`:
Output:
```
Some([[1000, 2000, 3000], [4000], [5000, 6000], [7000, 8000, 9000], [10000]])
```

### Advent of Code 2022 day 4

Input:
```
2-4,6-8
2-3,4-5
5-7,7-9
2-8,3-7
6-6,4-6
2-6,4-8
```

Code:
```rust
p.pos_int().many().until("\n").many().get()
```

Result `Option<Vec<Vec<u64>>>`:
```
Some([[2, 4, 6, 8], [2, 3, 4, 5], [5, 7, 7, 9], [2, 8, 3, 7], [6, 6, 4, 6], [2, 6, 4, 8]])
```

### Advent of Code 2022 day 11

Input:
```
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
```

Code:
```rust
struct Monkey {
    items: Vec<i64>,
    op: Vec<String>,
    on_true: i64,
    div: i64,
    on_false: i64,
}

p.map(|p| Some(Monkey{
    items: p.find("items").stay().int().many().until("\n").get()?,
    op: p.re_cap(r"old (.) (\w+)").get()?,
    div: p.find("divisible").int().get()?,
    on_true: p.find("true").int().get()?,
    on_false: p.find("false").int().get()?,
})).many().get()
```

Result `Option<Vec<Monkey>>`:
```
Some(
    [
        Monkey {
            items: [
                79,
                98,
            ],
            op: [
                "*",
                "19",
            ],
            on_true: 2,
            div: 23,
            on_false: 3,
        },
        Monkey {
            items: [
                54,
                65,
                75,
                74,
            ],
            op: [
                "+",
                "6",
            ],
            on_true: 2,
            div: 19,
            on_false: 0,
        },
        Monkey {
            items: [
                79,
                60,
                97,
            ],
            op: [
                "*",
                "old",
            ],
            on_true: 1,
            div: 13,
            on_false: 3,
        },
        Monkey {
            items: [
                74,
            ],
            op: [
                "+",
                "3",
            ],
            on_true: 0,
            div: 17,
            on_false: 1,
        },
    ],
)
```
