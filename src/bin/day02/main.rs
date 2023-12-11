#![feature(test)]
#![feature(type_alias_impl_trait)]
#![feature(anonymous_lifetime_in_impl_trait)]

#[derive(Debug, Clone, Copy)]
enum Color {
    Red,
    Green,
    Blue,
}

impl Color {
    fn max_amount(self) -> u32 {
        match self {
            Color::Red => 12,
            Color::Green => 13,
            Color::Blue => 14,
        }
    }
}

struct Cube {
    color: Color,
    amount: u32,
}

type RevealedCubeSet<'a> = impl Iterator<Item = Cube> + 'a;
type RevealedCubeSets<'a> = impl Iterator<Item = RevealedCubeSet<'a>> + 'a;

struct Game<'a> {
    id: u32,
    revealed_cube_sets: RevealedCubeSets<'a>,
}

#[derive(Debug, Clone, Copy, Default)]
struct MaxByColor {
    red: u32,
    green: u32,
    blue: u32,
}

impl MaxByColor {
    pub fn add_cube(mut self, cube: Cube) -> MaxByColor {
        let value = match cube.color {
            Color::Red => &mut self.red,
            Color::Green => &mut self.green,
            Color::Blue => &mut self.blue,
        };

        if cube.amount > *value {
            *value = cube.amount;
        }

        self
    }

    pub fn power(self) -> u32 {
        self.red * self.green * self.blue
    }
}

fn parse_inputs<'a>(input: &'a str) -> impl Iterator<Item = Game<'a>> {
    input.lines().map(|line| {
        let (left_side, right_side) = line.split_once(": ").unwrap();

        let (_, id) = left_side.split_once(' ').unwrap();
        let id = id.parse::<u32>().unwrap();

        let revealed_cube_sets = right_side.split("; ").map(|revealed_cube_sets| {
            revealed_cube_sets.split(", ").map(|revealed_cube_set| {
                let (amount, color) = revealed_cube_set.split_once(' ').unwrap();

                let color = match color {
                    "red" => Color::Red,
                    "green" => Color::Green,
                    "blue" => Color::Blue,
                    _ => unreachable!(),
                };

                let amount = amount.parse::<u32>().unwrap();

                Cube { color, amount }
            })
        });

        Game::<'a> {
            id,
            revealed_cube_sets,
        }
    })
}

fn part_1(input: &str) -> u32 {
    let games = parse_inputs(input);

    games
        .filter_map(|game| {
            let is_valid: bool = game
                .revealed_cube_sets
                .flatten()
                .all(|revealed_cube| revealed_cube.amount <= revealed_cube.color.max_amount());

            is_valid.then_some(game.id)
        })
        .sum()
}

fn part_2(input: &str) -> u32 {
    let games = parse_inputs(input);

    games
        .map(|game| {
            game.revealed_cube_sets
                .flatten()
                .fold(MaxByColor::default(), |max_by_color, cube| {
                    max_by_color.add_cube(cube)
                })
                .power()
        })
        .sum()
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
        assert_eq!(part_1(INPUT), 2377);
    }

    #[test]
    fn test_part_2() {
        assert_eq!(part_2(INPUT), 71220);
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
