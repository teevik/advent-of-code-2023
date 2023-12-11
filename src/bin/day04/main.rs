#![feature(test)]

struct Card {
    winning_numbers: [u8; 10],
    my_numbers: [u8; 25], // could be iterator
}

fn parse_numbers<const N: usize>(numbers: &str) -> [u8; N] {
    let mut numbers = numbers
        .split_ascii_whitespace()
        .filter(|number| !number.is_empty())
        .map(|number| number.parse::<u8>().unwrap());

    [(); N].map(|_| numbers.next().unwrap())
}

fn parse_input(input: &str) -> impl Iterator<Item = Card> + '_ {
    input.lines().map(|line| {
        let (_left_part, right_part) = line.split_once(": ").unwrap();

        let (winning_numbers, my_numbers) = right_part.split_once(" | ").unwrap();

        let winning_numbers = parse_numbers(winning_numbers);
        let my_numbers = parse_numbers(my_numbers);

        Card {
            winning_numbers,
            my_numbers,
        }
    })
}

fn part_1(input: &str) -> u32 {
    parse_input(input)
        .map(
            |Card {
                 winning_numbers,
                 my_numbers,
             }| {
                let mut points = 0;

                for number in my_numbers {
                    if winning_numbers.contains(&number) {
                        if points == 0 {
                            points = 1;
                        } else {
                            points *= 2;
                        }
                    }
                }

                points
            },
        )
        .sum()
}

fn part_2(input: &str) -> u64 {
    let games = parse_input(input);

    let mut amount_of_copies = Vec::new();

    for (index, game) in games.enumerate() {
        let copies = amount_of_copies.get(index).copied().unwrap_or_else(|| {
            amount_of_copies.push(1);
            1
        });

        let card_wins = game
            .my_numbers
            .into_iter()
            .filter(|number| game.winning_numbers.contains(number))
            .count();

        for index_offset in 1..=card_wins {
            let target_index = index + index_offset;

            if target_index >= amount_of_copies.len() {
                amount_of_copies.push(copies + 1)
            } else {
                amount_of_copies[target_index] += copies;
            }
        }
    }

    amount_of_copies.into_iter().sum()
}

fn main() {
    let input = include_str!("input.txt");

    let part_1 = part_1(input);
    dbg!(part_1);

    let part_2 = part_2(input);
    dbg!(part_2);
}

#[cfg(test)]
mod tests {
    extern crate test;
    use super::*;

    const INPUT: &str = include_str!("input.txt");

    #[test]
    fn test_part_1() {
        let part_1 = part_1(INPUT);
        assert_eq!(part_1, 26426);
    }

    #[test]
    fn test_part_2() {
        let part_2 = part_2(INPUT);
        assert_eq!(part_2, 6227972);
    }

    #[bench]
    fn bench_part_1(b: &mut test::Bencher) {
        b.iter(|| part_1(INPUT));
    }

    #[bench]
    fn bench_part_2(b: &mut test::Bencher) {
        b.iter(|| part_2(INPUT));
    }
}
