use serde_json::Value;
use std::cmp::Ordering;

const INPUT: &str = include_str!("input.txt");

#[derive(Eq, PartialEq, Debug)]
struct MyData(serde_json::value::Value);

impl PartialOrd for MyData {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        match (&self.0, &other.0) {
            (Value::Array(a), Value::Array(b)) => {
                for i in 0..a.len() {
                    let a = a.get(i).unwrap().clone();
                    let Some(b) = b.get(i) else {
                        return Some(Ordering::Greater);
                    };
                    match MyData(a).partial_cmp(&MyData(b.clone())) {
                        None => {
                            panic!("no comparison between {:?} and {:?}", self, other)
                        }
                        Some(Ordering::Equal) => {
                            continue;
                        }
                        Some(other) => {
                            return Some(other);
                        }
                    }
                }
                if a.len() == b.len() {
                    return Some(Ordering::Equal);
                }
                Some(Ordering::Less)
            }
            (Value::Number(a), Value::Number(b)) => {
                a.as_u64().unwrap().partial_cmp(&b.as_u64().unwrap())
            }
            (Value::Array(_), Value::Number(_)) => {
                self.partial_cmp(&MyData(Value::Array(vec![other.0.clone()])))
            }
            (Value::Number(_), Value::Array(_)) => {
                MyData(Value::Array(vec![self.0.clone()])).partial_cmp(&other)
            }
            (_, _) => {
                panic!("don't care for this one")
            }
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    const INPUT_TEST: &str = r"[1,1,3,1,1]
[1,1,5,1,1]

[[1],[2,3,4]]
[[1],4]

[9]
[[8,7,6]]

[[4,4],4,4]
[[4,4],4,4,4]

[7,7,7,7]
[7,7,7]

[]
[3]

[[[]]]
[[]]

[1,[2,[3,[4,[5,6,7]]]],8,9]
[1,[2,[3,[4,[5,6,0]]]],8,9]";

    #[test]
    fn it_parses() {
        let data = parse(INPUT_TEST);
        dbg!(data);
    }

    #[test]
    fn it_cmp() {
        let data = parse(INPUT_TEST);
        assert_eq!(data[0][0].partial_cmp(&data[0][1]), Some(Ordering::Less));
        assert_eq!(data[1][0].partial_cmp(&data[1][1]), Some(Ordering::Less));
        assert_eq!(data[2][0].partial_cmp(&data[2][1]), Some(Ordering::Greater));
        assert_eq!(data[3][0].partial_cmp(&data[3][1]), Some(Ordering::Less));
        assert_eq!(data[4][0].partial_cmp(&data[4][1]), Some(Ordering::Greater));
        assert_eq!(data[5][0].partial_cmp(&data[5][1]), Some(Ordering::Less));
        assert_eq!(data[6][0].partial_cmp(&data[6][1]), Some(Ordering::Greater));
        assert_eq!(data[7][0].partial_cmp(&data[7][1]), Some(Ordering::Greater));
    }
}

fn parse(input: &str) -> Vec<Vec<MyData>> {
    input
        .split("\n\n")
        .map(|x| {
            x.lines()
                .map(|l| MyData(serde_json::from_str(l).unwrap()))
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>()
}

#[test]
fn part1() {
    let data = parse(INPUT);
    let res = data
        .into_iter()
        .enumerate()
        .map(|(i, pair)| (i + 1, pair))
        .filter(|(_, pair)| pair[0].partial_cmp(&pair[1]) == Some(Ordering::Less))
        .fold(0, |acc, (i, _)| acc + i);
    println!("{:?}", res);
}
