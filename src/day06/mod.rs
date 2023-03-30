use std::collections::HashSet;

const INPUT: &str = include_str!("input.txt");

fn find_marker(input: &str, n_distinct_char: usize) -> usize {
    for i in n_distinct_char..input.len() {
        let marker = &input[i - n_distinct_char..i];
        let mut uniq = HashSet::new();
        if marker.chars().all(|c| uniq.insert(c)) {
            return i;
        }
    }
    panic!("No marker found")
}

#[test]
fn part1() {
    let result = find_marker(INPUT, 14);
    println!("{:?}", result);
}
