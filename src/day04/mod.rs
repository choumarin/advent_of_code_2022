const INPUT: &str = include_str!("input.txt");

#[derive(Debug)]
struct Range {
    start: usize,
    end: usize,
}

impl Range {
    fn contains(&self, other: &Range) -> bool {
        self.start <= other.start && self.end >= other.end
    }

    fn overlaps(&self, other: &Range) -> bool {
        (other.start <= self.start && self.start <= other.end)
            || (other.start <= self.end && self.end <= other.end)
            || (self.start <= other.start && other.start <= self.end)
            || (self.start <= other.end && other.end <= self.end)
    }
}

#[test]
fn test_contains() {
    assert!(Range { start: 1, end: 3 }.contains(&Range { start: 2, end: 3 }));
    assert!(!Range { start: 1, end: 3 }.contains(&Range { start: 2, end: 4 }));
}

#[test]
fn test_overlap() {
    assert!(Range { start: 1, end: 3 }.overlaps(&Range { start: 2, end: 4 }));
    assert!(Range { start: 1, end: 3 }.overlaps(&Range { start: 0, end: 1 }));
    assert!(Range { start: 20, end: 93 }.overlaps(&Range { start: 57, end: 92 }));
    assert!(!Range { start: 1, end: 3 }.overlaps(&Range { start: 5, end: 7 }));
    assert!(!Range { start: 5, end: 7 }.overlaps(&Range { start: 1, end: 3 }));
}

fn parse(input: &str) -> Vec<(Range, Range)> {
    input
        .lines()
        .map(|line| {
            let mut ranges = line
                .split(',')
                .map(|range_str| {
                    let mut parts = range_str.split('-');
                    Range {
                        start: parts
                            .next()
                            .expect("a start")
                            .parse()
                            .expect("a number repr"),
                        end: parts
                            .next()
                            .expect("an end")
                            .parse()
                            .expect("a number repr"),
                    }
                })
                .collect::<Vec<Range>>()
                .into_iter();
            (
                ranges.next().expect("a range"),
                ranges.next().expect("another range"),
            )
        })
        .collect()
}

#[test]
fn part1() {
    let input = parse(INPUT);
    let result = input
        .iter()
        .filter(|(range1, range2)| range1.contains(range2) || range2.contains(range1))
        .count();
    println!("{:?}", result);
}

#[test]
fn part2() {
    let input = parse(INPUT);
    let result = input
        .iter()
        .filter(|(range1, range2)| range1.overlaps(range2))
        .count();
    println!("{:?}", result);
}
