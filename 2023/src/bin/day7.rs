use std::{fs, collections::HashMap, cmp::Ordering};


#[derive(Eq)]
struct Hand {
    cards: String,
    bids: u32,
}

impl Ord for Hand {
    fn cmp(&self, other: &Self) -> Ordering {
        let compare = compare_hand(&self.cards, &other.cards);
        match compare {
            0 => Ordering::Equal,
            1 => Ordering::Greater,
            _ => Ordering::Less,
        }
    }
}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for Hand {
    fn eq(&self, other: &Self) -> bool {
        self.cards == other.cards
    }
}


fn get_hand_type(hand: &str) -> u32 {
    let mut cards: HashMap<char, u32> = HashMap::new();
    for card in hand.chars() {
        cards.entry(card).and_modify(|e| *e += 1).or_insert(1);
    }
    if cards.len() == 1 {
        return 6;
    } else if cards.len() == 2 {
        return cards.values().max().unwrap() + 1
    } else if cards.len() == 3 {
        return *cards.values().max().unwrap()
    }
    5 - (cards.len() as u32)
}

fn get_card_strength(card: &char) -> u32 {
    let strength: HashMap<char, u32> = HashMap::from([
        ('A', 14),
        ('K', 13),
        ('Q', 12),
        ('J', 11),
        ('T', 10),
    ]);
    if card.is_ascii_digit() {
        card.to_digit(10).unwrap()
    } else {
        *strength.get(card).unwrap()
    }
}

fn compare_same_type(left: &str, right: &str) -> i32 {
    for (left_card, right_card) in left.chars().zip(right.chars()) {
        let left_strength = get_card_strength(&left_card);
        let right_strength = get_card_strength(&right_card);
        if left_strength > right_strength {
            return 1;
        } else if left_strength < right_strength {
            return -1;
        }
    }
    0
}

fn compare_hand(left: &str, right: &str) -> i32 {
    if left != right {
        let left_type = get_hand_type(left);
        let right_type = get_hand_type(right);
        if left_type > right_type {
            return 1;
        } else if left_type < right_type {
            return -1
        }
        return compare_same_type(left, right);
    }
    0
}

fn part_one(text: &str) -> u32 {
    let mut hands: Vec<Hand> = Vec::new();

    for line in text.lines() {
        let split_line: Vec<&str> = line.split_whitespace().collect();
        hands.push(
            Hand{
                cards: split_line[0].to_string(),
                bids: split_line[1].parse().unwrap(),
            }
        );
    }

    hands.sort();
    let mut total = 0;
    for (index, hand) in hands.iter().enumerate() {
        total += hand.bids * (index as u32 + 1);
    }
    total
}

// fn part_two(text: &str) -> u64 {
// }

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = "32T3K 765
T55J5 684
KK677 28
KTJJT 220
QQQJA 483";

    #[test]
    fn test_part_one() {
        assert_eq!(part_one(TEST_INPUT), 6440);
    }

    // #[test]
    // fn test_part_two() {
    //     assert_eq!(part_two(TEST_INPUT), );
    // }
}

fn main() {
    let lines = fs::read_to_string("input/day7").expect("Can't read file");
    println!("{}", part_one(&lines));
    // println!("{}", part_two(&lines));
}
