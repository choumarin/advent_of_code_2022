use lazy_static::lazy_static;
use regex::Regex;
use std::collections::HashMap;

const INPUT: &str = include_str!("input.txt");

lazy_static! {
    static ref MOVE_REX: Regex =
        Regex::new(r"move (?P<count>\d+) from (?P<from>\d+) to (?P<to>\d+)").unwrap();
}

#[derive(Debug)]
struct Crate(char);

#[derive(Debug)]
struct Stack {
    id: usize,
    crates: Vec<Crate>,
}

#[derive(Debug, Eq, PartialEq)]
struct Move {
    from: usize,
    to: usize,
    count: usize,
}

impl From<&str> for Move {
    fn from(value: &str) -> Self {
        let captures = MOVE_REX
            .captures(value)
            .unwrap_or_else(|| panic!("invalid: {}", value));
        Move {
            from: captures
                .name("from")
                .expect("a from")
                .as_str()
                .parse::<usize>()
                .expect("a number str"),
            to: captures
                .name("to")
                .expect("a to")
                .as_str()
                .parse::<usize>()
                .expect("a number str"),
            count: captures
                .name("count")
                .expect("a count")
                .as_str()
                .parse::<usize>()
                .expect("a number str"),
        }
    }
}

#[test]
fn test_move_from() {
    assert_eq!(
        Move::from("move 2 from 5 to 9"),
        Move {
            from: 5,
            to: 9,
            count: 2
        }
    );
}

fn parse(input: &str) -> (HashMap<usize, Stack>, Vec<Move>) {
    let mut stack_lines = Vec::new();
    let mut move_lines = Vec::new();
    for line in input.lines() {
        if line.is_empty() {
            continue;
        }
        if line.starts_with("move") {
            move_lines.push(line);
        } else {
            stack_lines.push(line);
        }
    }
    let moves = move_lines
        .into_iter()
        .map(Move::from)
        .collect::<Vec<Move>>();

    stack_lines.reverse();
    let stack_indices = stack_lines.remove(0);
    let mut stacks = stack_indices
        .split_whitespace()
        .map(|s| Stack {
            id: s
                .trim()
                .parse::<usize>()
                .unwrap_or_else(|_| panic!("invalid value {:?}", s)),
            crates: Vec::new(),
        })
        .collect::<Vec<_>>();
    for line in stack_lines {
        for i in (0..line.len()).step_by(4) {
            let crate_str = &line[i..i + 2];
            if crate_str.starts_with("[") || crate_str.ends_with("]") {
                stacks[i / 4]
                    .crates
                    .push(Crate(crate_str.chars().skip(1).next().expect("a char")));
            }
        }
    }
    let stacks = stacks
        .into_iter()
        .map(|s| (s.id, s))
        .collect::<HashMap<_, _>>();

    (stacks, moves)
}

fn apply_moves(stacks: &mut HashMap<usize, Stack>, moves: &Vec<Move>) {
    for m in moves {
        for _ in 0..m.count {
            let t = stacks
                .get_mut(&m.from)
                .expect("crate id exists")
                .crates
                .pop()
                .expect("enough crates");
            stacks
                .get_mut(&m.to)
                .expect("crate id exists")
                .crates
                .push(t);
        }
    }
}

#[test]
fn part1() {
    let (mut stacks, moves) = parse(INPUT);
    apply_moves(&mut stacks, &moves);
    let mut result = stacks
        .iter()
        .map(|(key, stack)| stack)
        .collect::<Vec<&Stack>>();
    result.sort_unstable_by(|a, b| a.id.cmp(&b.id));
    let result = result
        .iter()
        .map(|s| s.crates.last().expect("at least a crate per stack").0)
        .collect::<String>();
    println!("{:?}", result);
}
