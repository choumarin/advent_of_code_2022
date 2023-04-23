use crate::day09::Direction::{Down, Left, Right, Up};
use std::collections::HashSet;
use std::fmt::{Display, Formatter};
use std::str::FromStr;

const INPUT: &str = include_str!("input.txt");

#[derive(Debug, Eq, PartialEq, Copy, Clone)]
enum Direction {
    Up,
    Down,
    Right,
    Left,
}

#[derive(Debug, Default, Eq, PartialEq, Copy, Clone, Hash)]
struct Point {
    x: i32,
    y: i32,
}

#[derive(Debug, Default)]
struct Rope {
    head: Point,
    tail: Point,
}

#[derive(Debug, Default)]
struct NewRope {
    pieces: [Rope; 10],
}

impl Display for NewRope {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let mut grid = [['-'; 20]; 20];
        grid[(self.pieces.first().unwrap().head.x + 10) as usize]
            [(self.pieces.first().unwrap().head.y + 10) as usize] = 'H';
        for (i, p) in self.pieces.iter().enumerate().skip(1) {
            let Point { x, y } = p.head;
            grid[(x + 10) as usize][(y + 10) as usize] = char::from_digit(i as u32, 10).unwrap();
        }
        for y in -10i32..10 {
            let y = -y;
            for x in -10i32..10 {
                write!(f, "{}", grid[(x + 10) as usize][(y + 9) as usize]).unwrap();
            }
            writeln!(f).unwrap();
        }
        Ok(())
    }
}

impl NewRope {
    fn appy_move(&mut self, d: Direction) {
        self.pieces[0].appy_move(d);
        for i in 0..self.pieces.len() {
            let prev = self.pieces.get(i).unwrap();
            let prev_tail = prev.tail;
            if let Some(next) = self.pieces.get_mut(i + 1) {
                next.head = prev_tail;
                next.update_tail();
            }
        }
    }
}

impl Rope {
    fn appy_move(&mut self, d: Direction) {
        self.head.move_in_direction(d);
        self.update_tail();
    }

    fn update_tail(&mut self) {
        let vec = self.vec();

        if vec.x.abs() > 1 && vec.y.abs() >= 1 || vec.x.abs() >= 1 && vec.y.abs() > 1 {
            self.tail.x += vec.x.signum();
            self.tail.y += vec.y.signum();
        } else if vec.x.abs() > 1 {
            self.tail.x += vec.x.signum();
        } else if vec.y.abs() > 1 {
            self.tail.y += vec.y.signum();
        }
    }

    fn vec(&self) -> Point {
        Point {
            x: self.head.x - self.tail.x,
            y: self.head.y - self.tail.y,
        }
    }

    fn len(&self) -> f64 {
        let v = self.vec();
        f64::from(v.x * v.x + v.y * v.y).sqrt()
    }
}

impl Point {
    fn move_in_direction(&mut self, d: Direction) {
        match d {
            Up => {
                self.y += 1;
            }
            Down => {
                self.y -= 1;
            }
            Right => {
                self.x += 1;
            }
            Left => {
                self.x -= 1;
            }
        }
    }
}

impl FromStr for Direction {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "U" => Ok(Up),
            "D" => Ok(Down),
            "R" => Ok(Right),
            "L" => Ok(Left),
            _ => Err(()),
        }
    }
}

#[derive(Debug, Eq, PartialEq)]
struct Move {
    direction: Direction,
    count: usize,
}

fn parse(input: &str) -> Vec<Move> {
    input
        .lines()
        .filter_map(|l| {
            let mut token = l.split(' ');
            Some(Move {
                direction: token.next()?.parse::<Direction>().ok()?,
                count: token.next()?.parse::<usize>().ok()?,
            })
        })
        .collect()
}

#[cfg(test)]
mod test {
    use super::*;

    const INPUT: &str = r"R 4
U 4
L 3
D 1
R 4
D 1
L 5
R 2";

    #[test]
    fn it_parses() {
        let moves = parse(INPUT);
        assert_eq!(
            moves,
            vec![
                Move {
                    direction: Right,
                    count: 4
                },
                Move {
                    direction: Up,
                    count: 4
                },
                Move {
                    direction: Left,
                    count: 3
                },
                Move {
                    direction: Down,
                    count: 1
                },
                Move {
                    direction: Right,
                    count: 4
                },
                Move {
                    direction: Down,
                    count: 1
                },
                Move {
                    direction: Left,
                    count: 5
                },
                Move {
                    direction: Right,
                    count: 2
                },
            ]
        );
    }

    #[test]
    fn it_moves() {
        let mut rope = Rope::default();
        rope.appy_move(Up);
        assert_eq!(rope.head.x, 0);
        assert_eq!(rope.head.y, 1);
        assert_eq!(rope.tail.x, 0);
        assert_eq!(rope.tail.y, 0);
        rope.appy_move(Right);
        assert_eq!(rope.head.x, 1);
        assert_eq!(rope.head.y, 1);
        assert_eq!(rope.tail.x, 0);
        assert_eq!(rope.tail.y, 0);
        rope.appy_move(Right);
        assert_eq!(rope.head.x, 2);
        assert_eq!(rope.head.y, 1);
        assert_eq!(rope.tail.x, 1);
        assert_eq!(rope.tail.y, 1);
    }

    const INPUT2: &str = r"R 5
U 8
L 8
D 3
R 17
D 10
L 25
U 20";

    #[test]
    fn it_moves2() {
        let moves = parse(INPUT2);

        let mut positions = HashSet::new();
        let mut rope = NewRope::default();
        for m in moves {
            println!("{:?}", m);
            println!("{}", rope);
            for i in 0..m.count {
                println!("{:?}", i);
                rope.appy_move(m.direction);
                positions.insert(rope.pieces.last().unwrap().head);
                println!("{}", rope);
                println!("{:?}", rope);
                // print_positions(&positions);
            }
        }
        assert_eq!(positions.len(), 36);
    }
}

fn print_positions(pos: &HashSet<Point>) {
    println!();
    for x in -10..10 {
        for y in -10..10 {
            if pos.contains(&Point { x, y }) {
                print!("#");
            } else {
                print!("-");
            }
        }
        println!();
    }
    println!();
}

#[test]
fn part1() {
    let moves = parse(INPUT);

    let mut positions = HashSet::new();
    let mut rope = Rope::default();
    for m in moves {
        for _ in 0..m.count {
            rope.appy_move(m.direction);
            positions.insert(rope.tail);
        }
    }
    println!("{:?}", positions.len());
}

#[test]
fn part2() {
    let moves = parse(INPUT);

    let mut positions = HashSet::new();
    let mut rope = NewRope::default();
    for m in moves {
        for _ in 0..m.count {
            rope.appy_move(m.direction);
            positions.insert(rope.pieces.last().unwrap().head);
        }
    }
    println!("{:?}", positions.len());
}
