use std::collections::HashSet;
use std::process::id;

const INPUT: &str = include_str!("input.txt");

struct Grid(Vec<Vec<usize>>);

fn parse(input: &str) -> Grid {
    Grid(
        input
            .lines()
            .map(|l| {
                l.chars()
                    .map(|c| c.to_digit(10).expect("a digit") as usize)
                    .collect()
            })
            .collect(),
    )
}

impl Grid {
    fn rows(&self) -> usize {
        self.0.len()
    }

    fn row(&self, idx: usize) -> Vec<usize> {
        self.0[idx].clone()
    }

    fn columns(&self) -> usize {
        self.0[0].len()
    }

    fn column(&self, idx: usize) -> Vec<usize> {
        self.0.iter().map(|r| r[idx]).collect()
    }

    fn visible_trees(&self) -> HashSet<(usize, usize)> {
        let mut set = HashSet::new();
        for row in 0..self.rows() {
            for col in visible_trees_in_vec(self.row(row)) {
                set.insert((row, col));
            }
        }
        for row in 0..self.rows() {
            let mut rev = self.row(row).clone();
            rev.reverse();
            for col in visible_trees_in_vec(rev) {
                set.insert((row, self.columns() - 1 - col));
            }
        }
        for col in 0..self.columns() {
            for row in visible_trees_in_vec(self.column(col)) {
                set.insert((row, col));
            }
        }
        for col in 0..self.columns() {
            let mut rev = self.column(col).clone();
            rev.reverse();
            for row in visible_trees_in_vec(rev) {
                set.insert((self.rows() - 1 - row, col));
            }
        }
        set
    }
}

fn visible_trees_in_vec(heights: Vec<usize>) -> Vec<usize> {
    let mut ret = vec![];
    let mut current_max_height = heights[0];
    ret.push(0);
    let mut idx = 1;
    while idx < heights.len() {
        if heights[idx] > current_max_height {
            ret.push(idx);
            current_max_height = heights[idx];
        }
        idx += 1;
    }
    ret
}

#[cfg(test)]
mod tests {
    use crate::day08::{parse, visible_trees_in_vec};
    use std::collections::HashSet;

    const INPUT: &str = r"30373
25512
65332
33549
35390";

    #[test]
    fn it_parses() {
        let grid = parse(INPUT);
        assert_eq!(grid.row(1), vec![2, 5, 5, 1, 2]);
        assert_eq!(grid.row(3), vec![3, 3, 5, 4, 9]);
        assert_eq!(grid.column(1), vec![0, 5, 5, 3, 5]);
        assert_eq!(grid.column(4), vec![3, 2, 2, 9, 0]);
    }

    #[test]
    fn it_sees_trees() {
        let grid = parse(INPUT);
        assert_eq!(visible_trees_in_vec(grid.row(0)), vec![0, 3]);
        assert_eq!(visible_trees_in_vec(grid.row(1)), vec![0, 1]);
    }

    #[test]
    fn it_sees_all_trees() {
        let grid = parse(INPUT);
        println!("{:?}", grid.visible_trees());
        assert_eq!(
            grid.visible_trees(),
            vec![
                (0, 0),
                (0, 1),
                (0, 2),
                (0, 3),
                (0, 4),
                (4, 0),
                (4, 1),
                (4, 2),
                (4, 3),
                (4, 4),
                (0, 0),
                (1, 0),
                (2, 0),
                (3, 0),
                (4, 0),
                (0, 4),
                (1, 4),
                (2, 4),
                (3, 4),
                (4, 4),
                (1, 1),
                (1, 2),
                (2, 1),
                (2, 3),
                (3, 2),
            ]
            .into_iter()
            .collect::<HashSet<_>>()
        )
    }
}

#[test]
fn part1() {
    let result = parse(INPUT);
    let result = result.visible_trees().iter().count();
    println!("{:?}", result);
}

#[test]
fn part2() {}
