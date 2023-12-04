use std::collections::HashSet;
use std::str::FromStr;

#[derive(Debug, Clone, PartialEq, Eq)]
struct Card {
    id: u32,
    winning: Vec<u32>,
    numbers: Vec<u32>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct CardParseError {}

impl FromStr for Card {
    type Err = CardParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let card = s.strip_prefix("Card ").ok_or(CardParseError {})?;

        let mut card_parts = card.split(':');
        let id: u32 = card_parts
            .next()
            .ok_or(CardParseError {})?
            .trim()
            .parse()
            .map_err(|_| CardParseError {})?;

        let card = card_parts.next().ok_or(CardParseError {})?;

        let mut card_parts = card.split('|');
        let winning: Vec<_> = card_parts
            .next()
            .ok_or(CardParseError {})?
            .split_whitespace()
            .map(u32::from_str)
            .map(Result::unwrap)
            .collect();

        let numbers: Vec<_> = card_parts
            .next()
            .ok_or(CardParseError {})?
            .split_whitespace()
            .map(u32::from_str)
            .map(Result::unwrap)
            .collect();

        Ok(Card {
            id,
            winning,
            numbers,
        })
    }
}

impl Card {
    fn score(&self) -> u32 {
        let matches = self.matches();
        if matches == 0 {
            0
        } else {
            1 << (self.matches() - 1)
        }
    }

    fn matches(&self) -> usize {
        let mut winning: HashSet<&u32> = self.winning.iter().collect();
        winning.retain(|v| self.numbers.contains(v));

        winning.len()
    }
}

///
///
/// #Argument
///
/// 'input' - The input.
pub fn part_one(input: &str) -> u32 {
    input
        .lines()
        .map(Card::from_str)
        .map(Result::unwrap)
        .map(|c| c.score())
        .sum()
}

///
///
/// #Argument
///
/// 'input' - The input.
pub fn part_two(input: &str) -> u32 {
    let cards: Vec<_> = input
        .lines()
        .map(Card::from_str)
        .map(Result::unwrap)
        .collect();

    let mut counts: Vec<u32> = vec![1; cards.len()];

    for (index, card) in cards.iter().enumerate() {
        let current_count = counts[index];

        let matches = card.matches();
        for _ in 0..current_count {
            for i in 0..matches {
                counts[index + i + 1] += 1;
            }
        }
    }

    counts.iter().sum()
}

#[cfg(test)]
mod tests {
    use crate::day04::*;

    #[test]
    fn parse_card() {
        assert_eq!(
            Card::from_str("Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53").unwrap(),
            Card {
                id: 1,
                winning: vec![41, 48, 83, 86, 17],
                numbers: vec![83, 86, 6, 31, 17, 9, 48, 53]
            }
        );
        assert_eq!(
            Card::from_str("Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1").unwrap(),
            Card {
                id: 3,
                winning: vec![1, 21, 53, 59, 44],
                numbers: vec![69, 82, 63, 72, 16, 21, 14, 1]
            }
        );
        assert_eq!(
            Card::from_str("Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11").unwrap(),
            Card {
                id: 6,
                winning: vec![31, 18, 13, 56, 72],
                numbers: vec![74, 77, 10, 23, 35, 67, 36, 11]
            }
        );
        assert_eq!(
            Card::from_str("Card   6: 92 39 18 64  7 71 48 29  3 38 | 55 29 73 31 15 75 13 71 94 48 78 23 54  7 10 86 34 82 91 85 67 14 57 64  3").unwrap(),
            Card { id: 6, winning: vec![92, 39, 18, 64, 7, 71, 48, 29, 3, 38], numbers: vec![55, 29, 73, 31, 15, 75, 13, 71, 94, 48, 78, 23, 54, 7, 10, 86, 34, 82, 91, 85, 67, 14, 57, 64, 3] }
        )
    }

    #[test]
    fn calculate_score() {
        assert_eq!(
            Card::from_str("Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53")
                .unwrap()
                .score(),
            8
        );
        assert_eq!(
            Card::from_str("Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19")
                .unwrap()
                .score(),
            2
        );
        assert_eq!(
            Card::from_str("Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1")
                .unwrap()
                .score(),
            2
        );
        assert_eq!(
            Card::from_str("Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83")
                .unwrap()
                .score(),
            1
        );
        assert_eq!(
            Card::from_str("Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36")
                .unwrap()
                .score(),
            0
        );
        assert_eq!(
            Card::from_str("Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11")
                .unwrap()
                .score(),
            0
        );
    }

    #[test]
    fn part_one_correct() {
        assert_eq!(
            part_one(
                "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11"
            ),
            13
        );
    }

    #[test]
    fn part_two_correct() {
        assert_eq!(
            part_two(
                "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11"
            ),
            30
        );
    }
}
