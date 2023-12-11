#![feature(test)]

use itertools::Itertools;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
enum Card {
    Joker,
    Card(u8),
    Jack,
    Queen,
    King,
    Ace,
}

type Hand = [Card; 5];

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
enum HandKind {
    HighCard(Hand),
    OnePair(Hand),
    TwoPair(Hand),
    ThreeOfAKind(Hand),
    FullHouse(Hand),
    FourOfAKind(Hand),
    FiveOfAKind(Hand),
}

fn parse_card(char: u8) -> Card {
    match char {
        b'A' => Card::Ace,
        b'K' => Card::King,
        b'Q' => Card::Queen,
        b'J' => Card::Jack,
        b'T' => Card::Card(10),
        b'9' => Card::Card(9),
        b'8' => Card::Card(8),
        b'7' => Card::Card(7),
        b'6' => Card::Card(6),
        b'5' => Card::Card(5),
        b'4' => Card::Card(4),
        b'3' => Card::Card(3),
        b'2' => Card::Card(2),
        _ => unreachable!(),
    }
}

fn parse_hand(hand: &[u8]) -> [Card; 5] {
    let hand = [hand[0], hand[1], hand[2], hand[3], hand[4]];

    hand.map(|card| parse_card(card))
}

fn get_hand_kind(hand: Hand) -> HandKind {
    let card_count = hand.iter().counts();
    let joker_amount = card_count.get(&Card::Joker).copied().unwrap_or(0);

    let card_count = card_count.values().sorted_unstable().collect_vec();

    use HandKind::*;

    let original_hand = match card_count.as_slice() {
        &[5] => FiveOfAKind(hand),
        &[1, 4] => FourOfAKind(hand),
        &[2, 3] => FullHouse(hand),
        &[1, 1, 3] => ThreeOfAKind(hand),
        &[1, 2, 2] => TwoPair(hand),
        &[1, 1, 1, 2] => OnePair(hand),
        &[1, 1, 1, 1, 1] => HighCard(hand),
        _ => unreachable!(),
    };

    match (original_hand, joker_amount) {
        (original_hand, 0) => original_hand,
        (FiveOfAKind(_), 5) => FiveOfAKind(hand),
        (FourOfAKind(_), 4) => FiveOfAKind(hand),
        (FourOfAKind(_), 1) => FiveOfAKind(hand),
        (FullHouse(_), 3) => FiveOfAKind(hand),
        (FullHouse(_), 2) => FiveOfAKind(hand),
        (ThreeOfAKind(_), 3) => FourOfAKind(hand),
        (ThreeOfAKind(_), 2) => FiveOfAKind(hand),
        (ThreeOfAKind(_), 1) => FourOfAKind(hand),
        (TwoPair(_), 2) => FourOfAKind(hand),
        (TwoPair(_), 1) => FullHouse(hand),
        (OnePair(_), 2) => ThreeOfAKind(hand),
        (OnePair(_), 1) => ThreeOfAKind(hand),
        (HighCard(_), 4) => FiveOfAKind(hand),
        (HighCard(_), 3) => FourOfAKind(hand),
        (HighCard(_), 2) => ThreeOfAKind(hand),
        (HighCard(_), 1) => OnePair(hand),

        _ => unreachable!(),
    }
}

#[derive(Debug)]
struct HandWithBid {
    hand: HandKind,
    bid: u32,
}

fn part_1(input: &str) -> u32 {
    let hands_with_bid = input.lines().map(|line| {
        let (hand, bid) = line.split_once(' ').unwrap();

        let hand = parse_hand(hand.as_bytes());
        let hand = get_hand_kind(hand);
        let bid = bid.parse::<u32>().unwrap();

        HandWithBid { hand, bid }
    });

    let sorted = hands_with_bid.sorted_unstable_by_key(|hand_with_bid| hand_with_bid.hand);

    let total_winnings = sorted
        .enumerate()
        .map(|(index, hand)| {
            let rank = index as u32 + 1;

            hand.bid * rank
        })
        .sum::<u32>();

    total_winnings
}

fn part_2(input: &str) -> u32 {
    let hands_with_bid = input.lines().map(|line| {
        let (hand, bid) = line.split_once(' ').unwrap();

        let hand = parse_hand(hand.as_bytes());
        let hand = hand.map(|card| {
            if card == Card::Jack {
                Card::Joker
            } else {
                card
            }
        });

        let hand = get_hand_kind(hand);
        let bid = bid.parse::<u32>().unwrap();

        HandWithBid { hand, bid }
    });

    let sorted = hands_with_bid.sorted_unstable_by_key(|hand_with_bid| hand_with_bid.hand);

    let total_winnings = sorted
        .enumerate()
        .map(|(index, hand)| {
            let rank = index as u32 + 1;

            hand.bid * rank
        })
        .sum::<u32>();

    total_winnings
}
fn main() {
    let input = include_str!("input.txt");

    dbg!(part_1(input));
    dbg!(part_2(input));
}

#[cfg(test)]
mod tests {
    extern crate test;
    use super::*;

    const INPUT: &str = include_str!("input.txt");

    #[test]
    fn test_part_1() {
        assert_eq!(part_1(INPUT), 249748283);
    }

    #[test]
    fn test_part_2() {
        assert_eq!(part_2(INPUT), 248029057);
    }

    #[bench]
    fn bench_part_1(b: &mut test::Bencher) {
        b.iter(|| part_1(INPUT));
    }

    #[bench]
    fn bench_part_2(b: &mut test::Bencher) {
        b.iter(|| part_2(INPUT));
    }

    #[test]
    fn test_kind_ord() {
        let rank_5 = get_hand_kind(parse_hand(b"QQQJA"));
        let rank_4 = get_hand_kind(parse_hand(b"T55J5"));
        let rank_3 = get_hand_kind(parse_hand(b"KK677"));
        let rank_2 = get_hand_kind(parse_hand(b"KTJJT"));
        let rank_1 = get_hand_kind(parse_hand(b"32T3K"));

        assert!(rank_5 > rank_4);
        assert!(rank_4 > rank_3);
        assert!(rank_3 > rank_2);
        assert!(rank_2 > rank_1);
    }
}
