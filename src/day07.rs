use std::cmp::Ordering;
use std::collections::HashMap;
use std::str::FromStr;

#[derive(Debug, Ord, PartialOrd, Eq, PartialEq)]
enum HandType {
    HighCard,
    OnePair,
    TwoPair,
    ThreeOfKind,
    FullHouse,
    FourOfKind,
    FiveOfKind,
}

impl FromStr for HandType {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.len() != 5 {
            return Err(String::from("Hands must be of length 5"));
        }

        let mut card_map: HashMap<char, i32> = HashMap::new();
        for c in s.chars() {
            let old_count = card_map.get(&c).unwrap_or(&0);
            card_map.insert(c, old_count + 1);
        }

        // Take out the wildcards...
        let wildcards = match card_map.len() {
            1 => 0,
            _ => *card_map.get(&'*').unwrap_or(&0),
        };

        if wildcards > 0 {
            card_map.remove(&'*');
        }

        let mut card_counts: Vec<_> = card_map.iter().collect();
        card_counts.sort_by(|(_, a), (_, b)| b.cmp(a));

        let mut card_count_0 = *card_counts[0].1;
        card_count_0 += wildcards;
        card_counts[0].1 = &card_count_0;

        match *card_counts[0].1 {
            5 => Ok(HandType::FiveOfKind),
            4 => Ok(HandType::FourOfKind),
            3 => match *card_counts[1].1 {
                2 => Ok(HandType::FullHouse),
                _ => Ok(HandType::ThreeOfKind),
            },
            2 => match *card_counts[1].1 {
                2 => Ok(HandType::TwoPair),
                _ => Ok(HandType::OnePair),
            },
            _ => Ok(HandType::HighCard),
        }
    }
}

#[derive(Debug, Eq, PartialEq)]
struct Hand {
    hand_type: HandType,
    hand: String,
    bid: i32,
}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Hand {
    fn cmp(&self, other: &Self) -> Ordering {
        if self.hand_type == other.hand_type {
            for (i, a) in self.hand.chars().enumerate() {
                let b = other.hand.chars().nth(i).unwrap();
                if a != b {
                    return Hand::get_value(a).cmp(&Hand::get_value(b));
                }
            }
            Ordering::Equal
        } else {
            self.hand_type.cmp(&other.hand_type)
        }
    }
}

impl FromStr for Hand {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut parts = s.split_whitespace();

        let hand = String::from(parts.next().unwrap());
        let hand_type: HandType = hand.parse()?;
        let bid: i32 = parts
            .next()
            .unwrap()
            .parse()
            .map_err(|_| "Error parsing bid")?;

        Ok(Hand {
            hand_type,
            hand,
            bid,
        })
    }
}

impl Hand {
    fn get_value(c: char) -> i32 {
        match c {
            '*' => 0,
            '2' => 2,
            '3' => 3,
            '4' => 4,
            '5' => 5,
            '6' => 6,
            '7' => 7,
            '8' => 8,
            '9' => 9,
            'T' => 10,
            'J' => 11,
            'Q' => 12,
            'K' => 13,
            'A' => 14,
            _ => -1,
        }
    }
}

///
///
/// #Argument
///
/// 'input' - The input.
pub fn part_one(input: &str) -> i32 {
    let mut hands: Vec<_> = input
        .lines()
        .map(|line| line.parse::<Hand>().unwrap())
        .collect();
    hands.sort();

    let mut winnings = 0;
    for (i, h) in hands.iter().enumerate() {
        winnings += ((i + 1) as i32) * h.bid;
    }

    winnings
}

///
///
/// #Argument
///
/// 'input' - The input.
pub fn part_two(input: &str) -> i32 {
    part_one(input.replace('J', "*").as_str())
}

#[cfg(test)]
mod tests {
    use crate::day07::*;

    #[test]
    fn hand_type_ordering() {
        assert!(HandType::FiveOfKind > HandType::FourOfKind);
        assert!(HandType::FourOfKind > HandType::FullHouse);
        assert!(HandType::FullHouse > HandType::ThreeOfKind);
        assert!(HandType::ThreeOfKind > HandType::TwoPair);
        assert!(HandType::TwoPair > HandType::OnePair);
        assert!(HandType::OnePair > HandType::HighCard);
    }

    #[test]
    fn hand_type_parsing() {
        assert_eq!("32T3K".parse::<HandType>(), Ok(HandType::OnePair));
        assert_eq!("T55J5".parse::<HandType>(), Ok(HandType::ThreeOfKind));
        assert_eq!("KK677".parse::<HandType>(), Ok(HandType::TwoPair));
        assert_eq!("KTJJT".parse::<HandType>(), Ok(HandType::TwoPair));
        assert_eq!("QQQJA".parse::<HandType>(), Ok(HandType::ThreeOfKind));

        assert_eq!("32T3K".parse::<HandType>(), Ok(HandType::OnePair));
        assert_eq!("T55*5".parse::<HandType>(), Ok(HandType::FourOfKind));
        assert_eq!("KK677".parse::<HandType>(), Ok(HandType::TwoPair));
        assert_eq!("KT**T".parse::<HandType>(), Ok(HandType::FourOfKind));
        assert_eq!("QQQ*A".parse::<HandType>(), Ok(HandType::FourOfKind));

        assert_eq!("*****".parse::<HandType>(), Ok(HandType::FiveOfKind));

        assert!("blah".parse::<HandType>().is_err());
    }

    #[test]
    fn hand_parsing() {
        assert_eq!(
            "32T3K 765".parse::<Hand>(),
            Ok(Hand {
                hand_type: HandType::OnePair,
                hand: String::from("32T3K"),
                bid: 765
            })
        );
    }

    #[test]
    fn part_one_correct() {
        assert_eq!(
            part_one(
                "32T3K 765
T55J5 684
KK677 28
KTJJT 220
QQQJA 483"
            ),
            6440
        );
    }

    #[test]
    fn part_two_correct() {
        assert_eq!(
            part_two(
                "32T3K 765
T55J5 684
KK677 28
KTJJT 220
QQQJA 483"
            ),
            5905
        );
    }
}
