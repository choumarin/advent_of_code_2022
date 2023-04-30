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
        s.sand.iter().any(|p| p.x == self.x && p.y > self.y)
            || s.rock.iter().any(|p| p.x == self.x && p.y > self.y)
    }
}

#[derive(Debug, Default)]
struct Structure {
    rock: HashSet<Point>,
    sand: HashSet<Point>,
    falling_grain: Option<Point>,
}

impl Display for Structure {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let mut min_x = self.rock.iter().min_by_key(|p| p.x).unwrap().x;
        let mut max_x = self.rock.iter().max_by_key(|p| p.x).unwrap().x;
        let mut min_y = self.rock.iter().min_by_key(|p| p.y).unwrap().y;
        let mut max_y = self.rock.iter().max_by_key(|p| p.y).unwrap().y;
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
                // println!("{:?} -> {:?}", segment[0], segment[1]);
                if segment[0].x == segment[1].x {
                    // vertical
                    let x = segment[0].x;
                    if segment[0].y <= segment[1].y {
                        for y in segment[0].y..=segment[1].y {
                            // print!("{:?} ", y);
                            s.rock.insert(Point { x, y });
                        }
                    } else {
                        for y in segment[1].y..=segment[0].y {
                            // print!("{:?} ", y);
                            s.rock.insert(Point { x, y });
                        }
                    }
                } else if segment[0].y == segment[1].y {
                    // horizontal
                    let y = segment[0].y;
                    if segment[0].x <= segment[1].x {
                        for x in segment[0].x..=segment[1].x {
                            // print!("{:?} ", x);
                            s.rock.insert(Point { x, y });
                        }
                    } else {
                        for x in segment[1].x..=segment[0].x {
                            // print!("{:?} ", x);
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
        s
    }

    fn accept(&self, grain: Point) -> bool {
        !self.rock.contains(&grain) && !self.sand.contains(&grain)
    }

    fn cycle(&mut self) {
        if let Some(grain) = self.falling_grain {
            if self.accept(grain.down()) {
                // dbg!(".");
                self.falling_grain = Some(grain.down());
            } else if self.accept(grain.left()) {
                // dbg!(".");
                self.falling_grain = Some(grain.left());
            } else if self.accept(grain.right()) {
                // dbg!(".");
                self.falling_grain = Some(grain.right());
            } else {
                // dbg!(".");
                self.sand.insert(grain);
                self.falling_grain = None;
            }
        } else {
            // dbg!(".");
            self.falling_grain = Some(SOURCE);
            self.cycle();
        }
    }

    fn is_stable(&self) -> bool {
        if let Some(grain) = self.falling_grain {
            !grain.will_settle(self)
        } else {
            false
        }
    }

    fn units_of_sand_until_stable(mut self) -> usize {
        while !self.is_stable() {
            self.cycle()
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
        println!("{structure}");
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
fn part2() {}
