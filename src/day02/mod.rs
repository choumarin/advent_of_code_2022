const INPUT: &str = include_str!("input.txt");

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
enum Hand {
    Rock,
    Paper,
    Scissors,
}

impl From<&str> for Hand {
    fn from(value: &str) -> Self {
        match value {
            "A" | "X" => Hand::Rock,
            "B" | "Y" => Hand::Paper,
            "C" | "Z" => Hand::Scissors,
            &_ => panic!("unknown value {value}")
        }
    }
}

impl From<usize> for Hand {
    fn from(value: usize) -> Self {
        match value {
            1 => Hand::Rock,
            2 => Hand::Paper,
            3 => Hand::Scissors,
            _ => panic!("unknown value {value}")
        }
    }
}

impl Hand {
    fn score(&self) -> usize {
        match self {
            Hand::Rock => 1,
            Hand::Paper => 2,
            Hand::Scissors => 3,
        }
    }

    fn fight(&self, other: &Hand) -> Outcome {
        if (other.score() % 3) + 1 == self.score() {
            Outcome::Win
        } else if other.score() == self.score() {
            Outcome::Tie
        } else {
            Outcome::Loss
        }
    }

    fn from_outcome(other_hand: &Hand, outcome: &Outcome) -> Hand {
        match outcome {
            Outcome::Tie => other_hand.clone(),
            Outcome::Win => ((other_hand.score() % 3) + 1).into(),
            Outcome::Loss => (((other_hand.score() + 1) % 3) + 1).into(),
        }
    }
}

#[test]
fn test_fight() {
    assert_eq!(Hand::Paper.fight(&Hand::Scissors), Outcome::Loss);
    assert_eq!(Hand::Paper.fight(&Hand::Rock), Outcome::Win);
    assert_eq!(Hand::Paper.fight(&Hand::Paper), Outcome::Tie);
    assert_eq!(Hand::Rock.fight(&Hand::Scissors), Outcome::Win);
    assert_eq!(Hand::Rock.fight(&Hand::Paper), Outcome::Loss);
    assert_eq!(Hand::Rock.fight(&Hand::Rock), Outcome::Tie);
    assert_eq!(Hand::Scissors.fight(&Hand::Scissors), Outcome::Tie);
    assert_eq!(Hand::Scissors.fight(&Hand::Rock), Outcome::Loss);
    assert_eq!(Hand::Scissors.fight(&Hand::Paper), Outcome::Win);
}

#[test]
fn test_from_outcome() {
    assert_eq!(Hand::from_outcome(&Hand::Rock, &Outcome::Win), Hand::Paper);
    assert_eq!(Hand::from_outcome(&Hand::Rock, &Outcome::Tie), Hand::Rock);
    assert_eq!(Hand::from_outcome(&Hand::Rock, &Outcome::Loss), Hand::Scissors);
    assert_eq!(Hand::from_outcome(&Hand::Paper, &Outcome::Win), Hand::Scissors);
    assert_eq!(Hand::from_outcome(&Hand::Paper, &Outcome::Tie), Hand::Paper);
    assert_eq!(Hand::from_outcome(&Hand::Paper, &Outcome::Loss), Hand::Rock);
    assert_eq!(Hand::from_outcome(&Hand::Scissors, &Outcome::Win), Hand::Rock);
    assert_eq!(Hand::from_outcome(&Hand::Scissors, &Outcome::Tie), Hand::Scissors);
    assert_eq!(Hand::from_outcome(&Hand::Scissors, &Outcome::Loss), Hand::Paper);
}

#[derive(Debug, Eq, PartialEq)]
enum Outcome {
    Win,
    Loss,
    Tie,
}

impl From<&str> for Outcome {
    fn from(value: &str) -> Self {
        match value {
            "X" => Outcome::Loss,
            "Y" => Outcome::Tie,
            "Z" => Outcome::Win,
            &_ => panic!("unknown value {value}")
        }
    }
}

impl Outcome {
    fn score(&self) -> usize {
        match self {
            Outcome::Win => 6,
            Outcome::Tie => 3,
            Outcome::Loss => 0,
        }
    }
}

fn parse1(input: &str) -> Vec<(Hand, Hand)> {
    input.lines().map(|line| {
        let mut hands = line.split_whitespace();
        (hands.next().expect("one hand").into(), hands.next().expect("second hand").into())
    }).collect()
}

fn parse2(input: &str) -> Vec<(Hand, Outcome)> {
    input.lines().map(|line| {
        let mut game = line.split_whitespace();
        (game.next().expect("other hand").into(), game.next().expect("outcome").into())
    }).collect()
}

#[test]
fn part1() {
    let hands = parse1(INPUT);
    let result = hands.iter().map(|(other_hand, my_hand)| { my_hand.fight(other_hand).score() + my_hand.score() }).sum::<usize>();
    println!("{:?}", result);
}

#[test]
fn part2() {
    let games = parse2(INPUT);
    let result = games.iter().map(|(other_hand, outcome)| {
        let my_hand = Hand::from_outcome(other_hand, outcome);
        my_hand.score() + outcome.score()
    }).sum::<usize>();
    println!("{:?}", result);
}

