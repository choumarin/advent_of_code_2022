use std::collections::{HashSet, VecDeque};

const INPUT: &str = include_str!("input.txt");

#[derive(Eq, PartialEq, Hash, Debug, Copy, Clone)]
struct Coord(usize, usize);

impl Coord {
    fn neighbors(&self, map: &Map) -> HashSet<Coord> {
        let mut n = HashSet::new();
        for x in -1i32..=1 {
            for y in -1i32..=1 {
                if x.abs() == y.abs() {
                    // no diagonals
                    continue;
                }
                let x = self.0 as i32 + x;
                let y = self.1 as i32 + y;
                if (x >= 0 || y >= 0) && map.at(Coord(x as usize, y as usize)).is_some() {
                    n.insert(Coord(x as usize, y as usize));
                }
            }
        }
        n
    }

    fn accessible_neighbors(&self, map: &Map) -> HashSet<Coord> {
        self.neighbors(map)
            .into_iter()
            .filter(|&n| map.at(n).unwrap().0 <= map.at(*self).unwrap().0 + 1)
            .collect()
    }
}

#[derive(Eq, PartialEq, Hash, Debug, Copy, Clone)]
struct Elevation(usize);

impl TryFrom<char> for Elevation {
    type Error = ();

    fn try_from(value: char) -> Result<Self, Self::Error> {
        if ('a'..='z').contains(&value) {
            Ok(Elevation(value as usize - 'a' as usize))
        } else {
            Err(())
        }
    }
}

#[derive(Debug)]
struct Map {
    elevations: Vec<Vec<Elevation>>,
    start: Coord,
    end: Coord,
}

impl Map {
    fn width(&self) -> usize {
        self.elevations[0].len()
    }
    fn hight(&self) -> usize {
        self.elevations.len()
    }
    fn at(&self, Coord(x, y): Coord) -> Option<Elevation> {
        self.elevations.get(y)?.get(x).copied()
    }
}

fn parse(input: &str) -> Map {
    let mut elevations = vec![];
    let mut start = Coord(0, 0);
    let mut end = Coord(0, 0);
    for (y, l) in input.lines().enumerate() {
        let mut l_v = vec![];
        for (x, mut c) in l.chars().enumerate() {
            match c {
                'S' => {
                    start = Coord(x, y);
                    c = 'a';
                }
                'E' => {
                    end = Coord(x, y);
                    c = 'z';
                }
                _ => (),
            }
            l_v.push(c.try_into().unwrap());
        }
        elevations.push(l_v);
    }
    Map {
        elevations,
        start,
        end,
    }
}

fn traverse(map: &Map) -> Option<usize> {
    let mut start_queue = VecDeque::new();
    start_queue.push_front(map.start);
    bfs(map, start_queue)
}

fn all_mins(map: &Map) -> Option<usize> {
    let mut start_queue = VecDeque::new();

    for (y, line) in map.elevations.iter().enumerate() {
        for (x, &e) in line.iter().enumerate() {
            if e == Elevation::try_from('a').unwrap() {
                start_queue.push_front(Coord(x, y));
            }
        }
    }

    bfs(map, start_queue)
}

fn bfs(map: &Map, mut queue: VecDeque<Coord>) -> Option<usize> {
    let mut visited = HashSet::new();
    let mut steps = 0;

    while !queue.is_empty() {
        // there's probably a way to avoid the copy here by just counting how many or using 2 queues and swapping
        for current in queue.drain(..).collect::<Vec<_>>() {
            if current == map.end {
                return Some(steps);
            }
            for n in current.accessible_neighbors(map) {
                if !visited.contains(&n) {
                    visited.insert(n);
                    queue.push_front(n);
                }
            }
        }
        steps += 1;
    }
    None
}

#[cfg(test)]
mod test {
    use crate::day12::{all_mins, parse, traverse, Coord, Elevation};

    const INPUT_TEST: &str = r"Sabqponm
abcryxxl
accszExk
acctuvwj
abdefghi";

    #[test]
    fn it_parses() {
        let map = parse(INPUT_TEST);
        assert_eq!(map.width(), 8);
        assert_eq!(map.hight(), 5);
        assert_eq!(map.start, Coord(0, 0));
        assert_eq!(map.end, Coord(5, 2));
        assert_eq!(map.at(map.start), Some(Elevation(0)));
        assert_eq!(map.at(map.end), Some(Elevation(25)));
        assert_eq!(map.at(Coord(1, 1)), Some(Elevation(1)));
        assert_eq!(
            Coord(0, 0).neighbors(&map),
            vec![Coord(0, 1), Coord(1, 0)].into_iter().collect()
        );
        assert_eq!(
            Coord(0, 2).accessible_neighbors(&map),
            vec![Coord(0, 1), Coord(0, 3)].into_iter().collect()
        )
    }

    #[test]
    fn it_traverses() {
        let map = parse(INPUT_TEST);
        assert_eq!(traverse(&map), Some(31));
    }

    #[test]
    fn it_starts_everywhere() {
        let map = parse(INPUT_TEST);
        assert_eq!(all_mins(&map), Some(29));
    }
}

#[test]
fn part1() {
    let map = parse(INPUT);
    println!("{:?}", traverse(&map));
}

#[test]
fn part2() {
    let map = parse(INPUT);
    println!("{:?}", all_mins(&map));
}
