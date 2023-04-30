use std::collections::HashSet;
use std::fmt::{Display, Formatter};

const INPUT: &str = include_str!("input.txt");

const SOURCE: Point = Point { x: 500, y: 0 };

#[derive(Debug, Default, Eq, PartialEq, Hash, Copy, Clone)]
struct Point {
    x: i32,
    y: i32,
}

impl Point {
    fn down(&self) -> Self {
        Self {
            x: self.x,
            y: self.y + 1,
        }
    }

    fn left(&self) -> Self {
        Self {
            x: self.x - 1,
            y: self.y + 1,
        }
    }

    fn right(&self) -> Self {
        Self {
            x: self.x + 1,
            y: self.y + 1,
        }
    }

    fn will_settle(&self, s: &Structure) -> bool {
        self.y <= s.max_y
    }
}

#[derive(Debug, Eq, PartialEq)]
enum Part {
    Part1,
    Part2,
}

impl Default for Part {
    fn default() -> Self {
        Part::Part1
    }
}

#[derive(Debug, Default)]
struct Structure {
    rock: HashSet<Point>,
    sand: HashSet<Point>,
    falling_grain: Option<Point>,
    max_y: i32,
    part: Part,
}

impl Display for Structure {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let mut min_x = self.rock.iter().min_by_key(|p| p.x).unwrap().x;
        let mut max_x = self.rock.iter().max_by_key(|p| p.x).unwrap().x;
        let mut min_y = self.rock.iter().min_by_key(|p| p.y).unwrap().y;
        let mut max_y = self.max_y;
        if !self.sand.is_empty() {
            min_x = min_x.min(self.sand.iter().min_by_key(|p| p.x).unwrap().x);
            max_x = max_x.max(self.sand.iter().max_by_key(|p| p.x).unwrap().x);
            min_y = min_y.min(self.sand.iter().min_by_key(|p| p.y).unwrap().y);
            max_y = max_y.max(self.sand.iter().max_by_key(|p| p.y).unwrap().y);
        }
        if let Some(grain) = self.falling_grain {
            min_x = min_x.min(grain.x);
            max_x = max_x.max(grain.x);
            min_y = min_y.min(grain.y);
            max_y = max_y.max(grain.y);
        }
        if self.part == Part::Part2 {
            max_y += 2;
        }

        for y in (min_y - 1)..=(max_y + 1) {
            for x in (min_x - 1)..=(max_x + 1) {
                if self.rock.contains(&Point { x, y }) {
                    write!(f, "#")?;
                } else if self.sand.contains(&Point { x, y })
                    || self.falling_grain == Some(Point { x, y })
                {
                    write!(f, "o")?;
                } else {
                    write!(f, ".")?;
                }
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

impl Structure {
    fn from(lines: Vec<Vec<Point>>) -> Self {
        let mut s = Structure::default();
        for line in lines {
            for segment in line.windows(2) {
                if segment[0].x == segment[1].x {
                    // vertical
                    let x = segment[0].x;
                    if segment[0].y <= segment[1].y {
                        for y in segment[0].y..=segment[1].y {
                            s.rock.insert(Point { x, y });
                        }
                    } else {
                        for y in segment[1].y..=segment[0].y {
                            s.rock.insert(Point { x, y });
                        }
                    }
                } else if segment[0].y == segment[1].y {
                    // horizontal
                    let y = segment[0].y;
                    if segment[0].x <= segment[1].x {
                        for x in segment[0].x..=segment[1].x {
                            s.rock.insert(Point { x, y });
                        }
                    } else {
                        for x in segment[1].x..=segment[0].x {
                            s.rock.insert(Point { x, y });
                        }
                    }
                } else {
                    panic!(
                        "Only vertical or horizontal lines. {:?} -> {:?}",
                        segment[0], segment[1]
                    );
                }
            }
        }
        s.max_y = s.rock.iter().max_by_key(|p| p.y).unwrap().y;
        s
    }

    fn accept(&self, grain: Point) -> bool {
        let mut accept = !self.rock.contains(&grain) && !self.sand.contains(&grain);
        if self.part == Part::Part2 {
            accept = accept && (grain.y < self.max_y + 2)
        }
        accept
    }

    fn cycle(&mut self) {
        if let Some(grain) = self.falling_grain {
            if self.accept(grain.down()) {
                self.falling_grain = Some(grain.down());
            } else if self.accept(grain.left()) {
                self.falling_grain = Some(grain.left());
            } else if self.accept(grain.right()) {
                self.falling_grain = Some(grain.right());
            } else {
                self.sand.insert(grain);
                self.falling_grain = None;
            }
        } else {
            self.falling_grain = Some(SOURCE);
            self.cycle();
        }
    }

    fn is_stable(&self) -> bool {
        match self.part {
            Part::Part1 => {
                if let Some(grain) = self.falling_grain {
                    !grain.will_settle(self)
                } else {
                    false
                }
            }
            Part::Part2 => self.sand.contains(&SOURCE),
        }
    }

    fn units_of_sand_until_stable(mut self) -> usize {
        while !self.is_stable() {
            self.cycle();
        }
        self.sand.len()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    const INPUT_TEST: &str = r"498,4 -> 498,6 -> 496,6
503,4 -> 502,4 -> 502,9 -> 494,9";

    #[test]
    fn it_parses() {
        let data = parse(INPUT_TEST);
        let structure = Structure::from(data);
        dbg!(structure);
    }

    #[test]
    fn it_print() {
        let data = parse(INPUT_TEST);
        let structure = Structure::from(data);
        println!("{structure}");
    }

    #[test]
    fn it_cycles() {
        let data = parse(INPUT_TEST);
        let mut structure = Structure::from(data);
        while !structure.is_stable() {
            structure.cycle();
            println!("{structure}");
        }
    }

    #[test]
    fn it_counts() {
        let mut structure = Structure::from(parse(INPUT_TEST));
        assert_eq!(structure.units_of_sand_until_stable(), 24);
    }

    #[test]
    fn it_cycles_part2() {
        let data = parse(INPUT_TEST);
        let mut structure = Structure::from(data);
        structure.part = Part::Part2;
        println!("{structure}");
        while !structure.is_stable() {
            structure.cycle();
            println!("{structure}");
        }
    }

    #[test]
    fn it_counts_part2() {
        let mut structure = Structure::from(parse(INPUT_TEST));
        structure.part = Part::Part2;
        assert_eq!(structure.units_of_sand_until_stable(), 93);
    }
}

fn parse(input: &str) -> Vec<Vec<Point>> {
    input
        .lines()
        .map(|l| {
            l.split(" -> ")
                .map(|s| {
                    let mut parts = s.split(',');
                    Point {
                        x: parts.next().unwrap().parse().unwrap(),
                        y: parts.next().unwrap().parse().unwrap(),
                    }
                })
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>()
}

#[test]
fn part1() {
    let structure = Structure::from(parse(INPUT));
    println!("{}", structure.units_of_sand_until_stable());
}

#[test]
fn part2() {
    let mut structure = Structure::from(parse(INPUT));
    structure.part = Part::Part2;
    println!("{}", structure.units_of_sand_until_stable());
}
