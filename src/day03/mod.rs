use std::collections::HashSet;

const INPUT: &str = include_str!("input.txt");

#[derive(Debug, Hash, Eq, PartialEq, Copy, Clone)]
struct Stuff(char);

impl From<char> for Stuff {
    fn from(value: char) -> Self {
        Stuff(value)
    }
}

impl Stuff {
    fn priority(&self) -> usize {
        if self.0.is_ascii_uppercase() {
            self.0 as usize - 'A' as usize + 27
        } else if self.0.is_ascii_lowercase() {
            self.0 as usize - 'a' as usize + 1
        } else {
            panic!("Not a valid char `{:?}`", self.0)
        }
    }
}

#[test]
fn test_priority() {
    assert_eq!(Stuff('A').priority(), 27);
    assert_eq!(Stuff('Z').priority(), 52);
    assert_eq!(Stuff('a').priority(), 1);
    assert_eq!(Stuff('z').priority(), 26);
}

#[derive(Debug)]
struct Sack(Vec<Stuff>, Vec<Stuff>);

impl From<(Vec<Stuff>, Vec<Stuff>)> for Sack {
    fn from(value: (Vec<Stuff>, Vec<Stuff>)) -> Self {
        Sack(value.0, value.1)
    }
}

impl Sack {
    fn duplicated(self) -> Stuff {
        let a = self.0.into_iter().collect::<HashSet<Stuff>>();
        let b = self.1.into_iter().collect::<HashSet<Stuff>>();
        a.intersection(&b)
            .copied()
            .collect::<Vec<Stuff>>()
            .remove(0)
    }
}

struct MergedSack(Vec<Stuff>);

impl MergedSack {
    fn from_sack(mut sack: Sack) -> MergedSack {
        sack.0.append(&mut (sack.1));
        MergedSack(sack.0)
    }
}

fn parse(input: &str) -> Vec<Sack> {
    input
        .lines()
        .map(|line| {
            let side_1 = line[..line.len() / 2]
                .chars()
                .map(|c| c.into())
                .collect::<Vec<Stuff>>();

            let side_2 = line[line.len() / 2..]
                .chars()
                .map(|c| c.into())
                .collect::<Vec<Stuff>>();
            (side_1, side_2).into()
        })
        .collect()
}

#[test]
fn part1() {
    let sacks = parse(INPUT);
    let sum_prios = sacks
        .into_iter()
        .map(|sack| sack.duplicated().priority())
        .sum::<usize>();
    println!("{:?}", sum_prios);
}

#[test]
fn part2() {
    let sacks = parse(INPUT);
    let merged_sacks = sacks
        .into_iter()
        .map(|s| MergedSack::from_sack(s))
        .collect::<Vec<_>>();
    let merged_prio_sum = merged_sacks
        .chunks(3)
        .map(|c| {
            let a = &c[0].0.iter().copied().collect::<HashSet<_>>();
            let b = &c[1].0.iter().copied().collect::<HashSet<_>>();
            let c = &c[2].0.iter().copied().collect::<HashSet<_>>();
            let i1 = a.intersection(&b).copied().collect::<HashSet<_>>();
            let i2 = b.intersection(&c).copied().collect::<HashSet<_>>();
            i1.intersection(&i2).copied().collect::<Vec<_>>()[0].priority()
        })
        .sum::<usize>();
    println!("{:?}", merged_prio_sum);
}
