#![feature(test)]

#[derive(Debug)]
struct Race {
    // Time in milliseconds
    time: u64,
    // Best distance in millimeters
    record_distance: u64,
}

impl Race {
    fn ways_to_beat_record(self) -> usize {
        let min_hold_time = (self.record_distance / self.time) + 1;

        let possible_hold_times = min_hold_time..self.time;

        possible_hold_times
            .filter(|hold_time| {
                let remaining_time = self.time - hold_time;
                let distance = remaining_time * hold_time;

                distance > self.record_distance
            })
            .count()
    }
}

fn part_1(input: &str) -> usize {
    let (times, distances) = input.split_once('\n').unwrap();

    let [times, distances] = [times, distances].map(|numbers| {
        numbers
            .split_ascii_whitespace()
            .skip(1)
            .map(|number| number.parse::<u64>().unwrap())
    });

    let races = times.zip(distances).map(|(time, record_distance)| Race {
        time,
        record_distance,
    });

    races
        .map(|race| race.ways_to_beat_record())
        .product::<usize>()
}

fn part_2(input: &str) -> usize {
    let (times, distances) = input.split_once('\n').unwrap();

    let [time, record_distance] = [times, distances].map(|numbers| {
        numbers
            .split_ascii_whitespace()
            .skip(1)
            .collect::<String>()
            .parse::<u64>()
            .unwrap()
    });

    let race = Race {
        time,
        record_distance,
    };

    race.ways_to_beat_record()
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
        let result = part_1(INPUT);
        assert_eq!(result, 800280);
    }

    #[test]
    fn test_part_2() {
        let result = part_2(INPUT);
        assert_eq!(result, 45128024);
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
