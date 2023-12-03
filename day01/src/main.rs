fn first_and_last_digit_in_line(line: &str) -> (u8, u8) {
    let (mut first, mut last) = (None, None);
    for c in line.chars() {
        if c.is_digit(10) {
            let n = c.to_digit(10).unwrap() as u8;
            if first.is_none() {
                first = Some(n);
            }
            last = Some(n);
        }
    }
    (first.unwrap(), last.unwrap())
}

fn advanced_first_and_last_digit_in_line(line: &str) -> (u8, u8) {
    let english_digits = vec!["zero", "one", "two", "three", "four", "five", "six", "seven", "eight", "nine"];

    let numberic_inds = line
        .char_indices()
        .filter(|(_, c)| c.is_digit(10))
        .map(|(i, _)| i)
        .collect::<Vec<usize>>();

    let mut english_dig_spans = vec![];
    for eng_dig in english_digits {
        let mut start = 0;
        while let Some(i) = line[start..].find(eng_dig) {
            let end = start + i + eng_dig.len();
            english_dig_spans.push((start, end));
            start = end;
        }
    }

    let max_span = english_dig_spans
        .iter()
        .chain(numberic_inds.iter().map(|i| (&i, &i)))

}

fn main() {
    let input = include_str!("../input.txt");
    let answer = input
        .lines()
        .map(first_and_last_digit_in_line)
        .map(|(first, last)| (first * 10 + last) as u32)
        .sum::<u32>();

    println!("Part 1 Answer: {}", answer);
}

#[test]
fn part1_test() {
    let test_input = include_str!("../test_input.txt");
    let answer = test_input
        .lines()
        .map(first_and_last_digit_in_line)
        .map(|(first, last)| (first * 10 + last) as u32)
        .sum::<u32>();

    assert_eq!(answer, 142);
}

