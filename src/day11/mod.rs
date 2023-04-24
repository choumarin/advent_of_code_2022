use crate::day11::Operation::*;
use crate::day11::Test::*;
use std::collections::HashMap;

const INPUT: &str = include_str!("input.txt");

#[derive(Eq, PartialEq, Debug, Copy, Clone)]
struct Item(usize);

#[derive(Eq, PartialEq, Debug, Copy, Clone)]
enum Operation {
    Add(usize),
    Multiply(usize),
    Square,
}

impl Item {
    fn operation(&mut self, op: Operation) {
        match op {
            Add(i) => self.0 += i,
            Multiply(i) => self.0 *= i,
            Square => self.0 *= self.0,
        }
    }

    fn test(&self, test: Test) -> bool {
        match test {
            DivisibleBy(i) => self.0 % i == 0,
        }
    }
}

#[derive(Eq, PartialEq, Debug, Copy, Clone)]
enum Test {
    DivisibleBy(usize),
}

#[derive(Eq, PartialEq, Debug)]
struct Monkey {
    inspections: usize,
    items: Vec<Item>,
    operation: Operation,
    test: Test,
    to_monkey_id_if_true: usize,
    to_monkey_id_if_false: usize,
}

impl Monkey {
    fn do_action_and_throw(&mut self) -> HashMap<usize, Vec<Item>> {
        let mut throws: HashMap<usize, Vec<_>> = HashMap::new();
        for mut item in self.items.drain(..) {
            self.inspections += 1;
            throws
                .entry({
                    item.operation(self.operation);
                    item.0 /= 3;
                    if item.test(self.test) {
                        self.to_monkey_id_if_true
                    } else {
                        self.to_monkey_id_if_false
                    }
                })
                .or_default()
                .push(item);
        }
        throws
    }

    fn receive(&mut self, mut items: Vec<Item>) {
        self.items.append(&mut items);
    }
}

fn distribute(monkeys: &mut HashMap<usize, Monkey>, mut throws: HashMap<usize, Vec<Item>>) {
    for (m_id, items) in throws.drain() {
        monkeys
            .get_mut(&m_id)
            .unwrap_or_else(|| {
                panic!("no monkey id {m_id}");
            })
            .receive(items);
    }
}

fn do_one_round(monkeys: &mut HashMap<usize, Monkey>) {
    // this cheats around the double borrow, but we need to process in order
    for i in 0..monkeys.len() {
        let throw = monkeys.get_mut(&i).unwrap().do_action_and_throw();
        distribute(monkeys, throw);
    }
}

use lazy_static::lazy_static;
use regex::Regex;

lazy_static! {
    static ref ID_REX: Regex = Regex::new(r"Monkey (?P<id>\d+):").unwrap();
    static ref DIV_REX: Regex = Regex::new(r"divisible by (?P<num>\d+)").unwrap();
    static ref MUL_REX: Regex = Regex::new(r"new = old \* (?P<num>\d+)").unwrap();
    static ref ADD_REX: Regex = Regex::new(r"new = old \+ (?P<num>\d+)").unwrap();
    static ref THR_REX: Regex = Regex::new(r"throw to monkey (?P<id>\d+)").unwrap();
}

fn parse(input: &str) -> HashMap<usize, Monkey> {
    input
        .split("\n\n")
        .map(|m| {
            let mut lines = m.lines();
            let id_line = lines.next().unwrap();
            let captures = ID_REX.captures(id_line).unwrap();
            let id = captures.name("id").unwrap().as_str().parse().unwrap();

            let items_line = lines.next().unwrap();
            let items = items_line
                .split(':')
                .nth(1)
                .unwrap()
                .split(',')
                .map(|n| Item(n.trim().parse().unwrap()))
                .collect();

            let operation_str = lines.next().unwrap().split(':').nth(1).unwrap().trim();
            let operation;
            if let Some(captures) = ADD_REX.captures(operation_str) {
                operation = Add(captures.name("num").unwrap().as_str().parse().unwrap());
            } else if let Some(captures) = MUL_REX.captures(operation_str) {
                operation = Multiply(captures.name("num").unwrap().as_str().parse().unwrap());
            } else if "new = old * old" == operation_str {
                operation = Square;
            } else {
                panic!("Unknown operation: `{}`", operation_str);
            }

            let test_str = lines.next().unwrap().split(':').nth(1).unwrap();
            let captures = DIV_REX.captures(test_str).unwrap();
            let test = DivisibleBy(captures.name("num").unwrap().as_str().parse().unwrap());

            let mut to_monkey_id_if_true_str = lines.next().unwrap().split(':');
            assert_eq!(to_monkey_id_if_true_str.next().unwrap().trim(), "If true");
            let captures = THR_REX
                .captures(to_monkey_id_if_true_str.next().unwrap())
                .unwrap();
            let to_monkey_id_if_true = captures.name("id").unwrap().as_str().parse().unwrap();

            let mut to_monkey_id_if_false_str = lines.next().unwrap().split(':');
            assert_eq!(to_monkey_id_if_false_str.next().unwrap().trim(), "If false");
            let captures = THR_REX
                .captures(to_monkey_id_if_false_str.next().unwrap())
                .unwrap();
            let to_monkey_id_if_false = captures.name("id").unwrap().as_str().parse().unwrap();

            (
                id,
                Monkey {
                    inspections: 0,
                    items,
                    operation,
                    test,
                    to_monkey_id_if_true,
                    to_monkey_id_if_false,
                },
            )
        })
        .collect()
}

#[cfg(test)]
mod test {
    use super::*;

    const INPUT_TEST: &str = r"Monkey 0:
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

    #[test]
    fn it_parses() {
        let monkeys = parse(INPUT_TEST);
        assert_eq!(
            monkeys.get(&0),
            Some(&Monkey {
                inspections: 0,
                items: vec![Item(79), Item(98)],
                operation: Multiply(19),
                test: DivisibleBy(23),
                to_monkey_id_if_true: 2,
                to_monkey_id_if_false: 3,
            })
        );
        assert_eq!(
            monkeys.get(&3),
            Some(&Monkey {
                inspections: 0,
                items: vec![Item(74)],
                operation: Add(3),
                test: DivisibleBy(17),
                to_monkey_id_if_true: 0,
                to_monkey_id_if_false: 1,
            })
        )
    }

    #[test]
    fn it_throws() {
        let mut monkeys = parse(INPUT_TEST);
        let m0 = monkeys.get_mut(&0).unwrap();
        let throw0 = m0.do_action_and_throw();
        assert_eq!(
            throw0
                .clone()
                .into_iter()
                .collect::<Vec<(usize, Vec<Item>)>>(),
            vec![(3, vec![Item(500), Item(620)])]
        );
        assert!(m0.items.is_empty());
        assert_eq!(m0.inspections, 2);
        distribute(&mut monkeys, throw0);
        assert_eq!(
            monkeys.get(&3).unwrap().items,
            vec![Item(74), Item(500), Item(620)]
        );
    }

    #[test]
    fn one_round() {
        let mut monkeys = parse(INPUT_TEST);
        do_one_round(&mut monkeys);
        assert_eq!(
            monkeys.get(&0).unwrap().items,
            vec![Item(20), Item(23), Item(27), Item(26)]
        );
        assert_eq!(
            monkeys.get(&1).unwrap().items,
            vec![
                Item(2080),
                Item(25),
                Item(167),
                Item(207),
                Item(401),
                Item(1046)
            ]
        );
        assert!(monkeys.get(&2).unwrap().items.is_empty());
        assert!(monkeys.get(&3).unwrap().items.is_empty());
    }
}

#[test]
fn part1() {
    let mut monkeys = parse(INPUT);

    for _ in 0..20 {
        do_one_round(&mut monkeys);
    }
    let mut actives = monkeys.values().map(|m| m.inspections).collect::<Vec<_>>();
    actives.sort_unstable();
    actives.reverse();
    let res = actives.into_iter().take(2).reduce(|a, b| a * b).unwrap();
    println!("{}", res);
}
