use std::{cmp::Ordering, collections::BTreeMap};

pub fn parse(input: &str) -> u64 {
    let mut hands = input
        .lines()
        .map(|line| {
            let (cards, bid) = line.split_once(' ').unwrap();
            let cards = cards.as_bytes();
            let hand: [CardKind; 5] = std::array::from_fn(|i| cards[i].into());
            (hand, hand.into(), bid.parse::<u64>().unwrap())
        })
        .collect::<Vec<([CardKind; 5], HandKind, u64)>>();

    hands.sort_by(|(hand1, hand_kind1, _), (hand2, hand_kind2, _)| {
        match hand_kind1.cmp(hand_kind2) {
            Ordering::Equal => hand1.cmp(hand2),
            other => other,
        }
    });

    hands
        .into_iter()
        .enumerate()
        .map(|(i, (_, _, bid))| bid * (i as u64 + 1))
        .sum()
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
enum CardKind {
    CardJ,
    Card2,
    Card3,
    Card4,
    Card5,
    Card6,
    Card7,
    Card8,
    Card9,
    CardT,
    CardQ,
    CardK,
    CardA,
}

impl From<u8> for CardKind {
    fn from(value: u8) -> Self {
        match value {
            b'2' => CardKind::Card2,
            b'3' => CardKind::Card3,
            b'4' => CardKind::Card4,
            b'5' => CardKind::Card5,
            b'6' => CardKind::Card6,
            b'7' => CardKind::Card7,
            b'8' => CardKind::Card8,
            b'9' => CardKind::Card9,
            b'T' => CardKind::CardT,
            b'J' => CardKind::CardJ,
            b'Q' => CardKind::CardQ,
            b'K' => CardKind::CardK,
            b'A' => CardKind::CardA,
            _ => panic!("Invalid card kind"),
        }
    }
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
enum HandKind {
    HighCard,
    OnePair,
    TwoPair,
    ThreeOfAKind,
    FullHouse,
    FourOfAKind,
    FiveOfAKind,
}

impl From<[CardKind; 5]> for HandKind {
    fn from(hand: [CardKind; 5]) -> Self {
        let mut card_count_map: BTreeMap<CardKind, u8> = BTreeMap::new();

        for card in hand {
            card_count_map
                .entry(card)
                .and_modify(|count| *count += 1)
                .or_insert(1);
        }

        let joker_count = card_count_map.remove(&CardKind::CardJ).unwrap_or(0);
        let mut card_counts: Vec<u8> = card_count_map.into_values().collect();
        card_counts.sort_by(|a, b| b.cmp(a));

        if let Some(count) = card_counts.get_mut(0) {
            *count += joker_count;
        } else {
            card_counts.push(joker_count);
        };

        match card_counts.as_slice() {
            [5] => HandKind::FiveOfAKind,
            [4, 1] => HandKind::FourOfAKind,
            [3, 2] => HandKind::FullHouse,
            [3, 1, 1] => HandKind::ThreeOfAKind,
            [2, 2, 1] => HandKind::TwoPair,
            [2, 1, 1, 1] => HandKind::OnePair,
            _ => HandKind::HighCard,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_example() {
        let example_input = std::fs::read_to_string("../../../input/day07_example.txt")
            .expect("example file should exist");
        let result = parse(&example_input);
        assert_eq!(result, 5905);
    }
}
