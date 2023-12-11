#![feature(test)]

struct CalibrationDigits {
    first_digit: u8,
    last_digit: u8,
}

fn parse_part1(input: &str) -> impl Iterator<Item = CalibrationDigits> + '_ {
    input.lines().map(|line| {
        let line = line.as_bytes();

        fn find_first_digit(mut line: impl Iterator<Item = u8>) -> u8 {
            line.find(|&char| char <= b'9').unwrap() - b'0'
        }

        let first_digit = find_first_digit(line.iter().copied());
        let last_digit = find_first_digit(line.iter().copied().rev());

        CalibrationDigits {
            first_digit,
            last_digit,
        }
    })
}

fn find_word_digit(word: &[u8]) -> Option<u8> {
    const WORD_DIGITS: [(&[u8], u8); 9] = [
        (b"one", 1),
        (b"two", 2),
        (b"three", 3),
        (b"four", 4),
        (b"five", 5),
        (b"six", 6),
        (b"seven", 7),
        (b"eight", 8),
        (b"nine", 9),
    ];

    let length = word.len();

    for (target_word, digit) in WORD_DIGITS {
        if length < target_word.len() {
            continue;
        }

        let is_match = target_word
            .iter()
            .zip(word.iter())
            .all(|(&target_char, &char)| target_char == char);

        if is_match {
            return Some(digit);
        }
    }

    None
}

fn parse_part2(input: &str) -> impl Iterator<Item = CalibrationDigits> + '_ {
    input.lines().map(|line| {
        let line = line.as_bytes();

        let find_digit_at_index = |index: usize| {
            let digit_char = line[index];

            if digit_char <= b'9' {
                return Some(digit_char - b'0');
            }

            if let Some(word_digit) = find_word_digit(&line[index..]) {
                return Some(word_digit);
            }

            None
        };

        let first_digit = (0..line.len()).find_map(find_digit_at_index).unwrap();
        let last_digit = (0..line.len()).rev().find_map(find_digit_at_index).unwrap();

        CalibrationDigits {
            first_digit,
            last_digit,
        }
    })
}

fn calculate(digits: impl Iterator<Item = CalibrationDigits>) -> u64 {
    digits
        .map(
            |CalibrationDigits {
                 first_digit,
                 last_digit,
             }| first_digit * 10 + last_digit,
        )
        .map(|number| number as u64)
        .sum()
}

fn main() {
    let input = include_str!("input.txt");

    let part_1 = calculate(parse_part1(input));
    dbg!(part_1);

    let part_2 = calculate(parse_part2(input));
    dbg!(part_2);
}

#[cfg(test)]
mod tests {
    extern crate test;
    use super::*;

    const INPUT: &str = include_str!("input.txt");

    #[test]
    fn test_part1() {
        let result = calculate(parse_part1(INPUT));
        assert_eq!(result, 54601);
    }

    #[test]
    fn test_part2() {
        let result = calculate(parse_part2(INPUT));
        assert_eq!(result, 54078);
    }

    #[bench]
    fn bench_part1(b: &mut test::Bencher) {
        b.iter(|| calculate(parse_part1(INPUT)));
    }

    #[bench]
    fn bench_part2(b: &mut test::Bencher) {
        b.iter(|| calculate(parse_part2(INPUT)));
    }
}
