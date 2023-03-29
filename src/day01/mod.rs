const INPUT: &str = include_str!("input.txt");

#[derive(Debug)]
struct Elf {
    calories: Vec<usize>,
}

impl Elf {
    fn total_calories(&self) -> usize {
        self.calories.iter().sum()
    }
}

fn parse(input: &str) -> Vec<Elf> {
    input
        .split("\r\n\r\n")
        .map(|lines| Elf {
            calories: lines
                .lines()
                .map(|line| line.parse().expect("a number"))
                .collect(),
        })
        .collect()
}

#[test]
fn part1() {
    let elves = parse(INPUT);
    println!("{:?}", elves);
    let mut elves = elves
        .iter()
        .map(Elf::total_calories)
        .collect::<Vec<usize>>();
    println!("{:?}", elves);
    elves.sort_unstable();
    elves.reverse();
    println!("{:?}", elves);
    let result = elves.first().expect("at least one elf");
    println!("{:?}", result);
}

#[test]
fn part2() {
    let elves = parse(INPUT);
    println!("{:?}", elves);
    let mut elves = elves
        .iter()
        .map(Elf::total_calories)
        .collect::<Vec<usize>>();
    println!("{:?}", elves);
    elves.sort_unstable();
    elves.reverse();
    println!("{:?}", elves);
    let result = elves.iter().take(3).sum::<usize>();
    println!("{:?}", result);
}
