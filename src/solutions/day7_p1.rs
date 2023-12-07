use std::collections::HashMap;

const CARD_ORDER: &[char] = &[
    '2', '3', '4', '5', '6', '7', '8', '9', 'T', 'J', 'Q', 'K', 'A',
];

struct Hand<'a> {
    hand: &'a str,
    bid: u32,
    hand_power: u32,
}

pub fn solution(lines: Vec<&str>) -> String {
    let mut hands: Vec<Hand> = vec![];

    for line in lines {
        let (hand, bid) = line.split_once(' ').unwrap();

        let mut hand_map: HashMap<char, u32> = HashMap::new();
        for ch in hand.chars() {
            match hand_map.get_mut(&ch) {
                Some(v) => *v += 1,
                None => {
                    hand_map.insert(ch, 1);
                }
            }
        }

        let bid = bid.parse::<u32>().unwrap();

        let max_same_cards = hand_map.values().max().unwrap();

        let hand_power = match max_same_cards {
            1 => 0,
            2 => match hand_map.len() {
                4 => 1,
                3 => 2,
                _ => unreachable!(),
            },
            3 => match hand_map.len() {
                3 => 3,
                2 => 4,
                _ => unreachable!(),
            },
            4 => 5,
            5 => 6,
            _ => unreachable!(),
        };

        hands.push(Hand {
            hand,
            bid,
            hand_power,
        });
    }

    hands.sort_by(|a, b| {
        if a.hand_power != b.hand_power {
            return a.hand_power.cmp(&b.hand_power);
        }

        for (a, b) in a.hand.chars().zip(b.hand.chars()) {
            if a != b {
                let a_order = CARD_ORDER.iter().position(|x| x == &a).unwrap();
                let b_order = CARD_ORDER.iter().position(|x| x == &b).unwrap();
                return a_order.cmp(&b_order);
            }
        }

        return std::cmp::Ordering::Equal;
    });

    let mut sum = 0;

    for (i, hand) in hands.iter().enumerate() {
        sum += (i as u32 + 1) * hand.bid;
    }

    sum.to_string()
}
