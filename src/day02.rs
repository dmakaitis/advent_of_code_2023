use std::str::FromStr;

/// Representation of counts of cube by color that are pulled from the bag.
#[derive(Debug, PartialEq)]
struct ColorCount {
    red: u32,
    green: u32,
    blue: u32,
}

impl ColorCount {
    /// Constructs a color count with the given values.
    ///
    /// # Arguments
    ///
    /// 'red' - The red value
    ///
    /// 'green' - The green value
    ///
    /// 'blue' The blue value
    fn _new(red: u32, green: u32, blue: u32) -> ColorCount {
        ColorCount { red, green, blue }
    }

    /// Returns true if the given result could have been pulled from a bad containing exactly
    /// the counts contained in this object.
    ///
    /// # Arguments
    ///
    /// 'other' - the dice count to check to see if it could have been pulled from a bag containing
    /// this dice count.
    fn is_possible(&self, other: &ColorCount) -> bool {
        other.red <= self.red && other.green <= self.green && other.blue <= self.blue
    }

    /// Calculates the power of a collection of color counts as defined by product of the maximum
    /// red, green, and blue values from all color counts in the given collection.
    ///
    /// # Arguments
    ///
    /// 'results' - the collection of color counts for which to calculate the power.
    fn calculate_power(color_counts: Vec<ColorCount>) -> u32 {
        let mut max_counts = ColorCount {
            red: 0,
            green: 0,
            blue: 0,
        };

        for count in color_counts {
            if count.red > max_counts.red {
                max_counts.red = count.red;
            }
            if count.green > max_counts.green {
                max_counts.green = count.green;
            }
            if count.blue > max_counts.blue {
                max_counts.blue = count.blue;
            }
        }

        max_counts.red * max_counts.green * max_counts.blue
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct ColorCountParseError {}

impl FromStr for ColorCount {
    type Err = ColorCountParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut red = 0;
        let mut green = 0;
        let mut blue = 0;

        let parts = s.split(',');
        for part in parts {
            let mut values = part.split_whitespace();
            let count: u32 = values.next().unwrap().parse().unwrap();
            let color = values.next().unwrap();

            match color {
                "red" => {
                    red += count;
                }
                "green" => {
                    green += count;
                }
                "blue" => {
                    blue += count;
                }
                _ => {
                    return Err(ColorCountParseError {});
                }
            }
        }

        Ok(ColorCount { red, green, blue })
    }
}

/// Returns the ID if the game described in the given input, or None if it could not be found.
///
/// # Argument
///
/// 'text' - The game results from which to extract the ID.
fn get_game_id(text: &str) -> Option<u32> {
    let text = text.strip_prefix("Game ")?;
    let index = text.find(':')?;
    text[..index].parse().ok()
}

/// Represents a game result, including its identifier and color counts from each cube pull.
#[derive(Debug, PartialEq)]
struct Game {
    id: u32,
    results: Vec<ColorCount>,
}

#[derive(Debug, PartialEq, Eq, Clone)]
struct GameParseError {}

impl FromStr for Game {
    type Err = GameParseError;

    fn from_str(text: &str) -> Result<Self, Self::Err> {
        let id = get_game_id(text).ok_or(GameParseError {})?;

        let text = text
            .strip_prefix(&format!("Game {id}: "))
            .ok_or(GameParseError {})?;

        let results = text.split(';');
        let results: Vec<_> = results
            .map(ColorCount::from_str)
            .map(Result::unwrap)
            .collect();

        Ok(Game { id, results })
    }
}

/// Returns the sum of the game ID's for all game results that would be possible if the bag
/// contained 12 red, 13 green, and 14 blue cubes.
///
/// # Arguments
///
/// 'input' - The input text containing the results of all the games.
pub fn part_one(input: &str) -> u32 {
    let max_cubes = ColorCount {
        red: 12,
        green: 13,
        blue: 14,
    };

    input
        .lines()
        .map(Game::from_str)
        .map(Result::unwrap)
        .filter(|game| {
            game.results
                .iter()
                .all(|result| max_cubes.is_possible(result))
        })
        .map(|game| game.id)
        .sum()
}

/// Calculates the sum of the power of all the results from all of the games played.
///
/// # Arguments
///
///
/// 'input' - The input text containing the results of all the games.
pub fn part_two(input: &str) -> u32 {
    input
        .lines()
        .map(Game::from_str)
        .map(Result::unwrap)
        .map(|game| ColorCount::calculate_power(game.results))
        .sum()
}

#[cfg(test)]
mod tests {
    use crate::day02::*;

    #[test]
    fn parsing_color_counts() {
        assert_eq!(
            ColorCount::from_str("3 blue, 4 red").unwrap(),
            ColorCount {
                blue: 3,
                red: 4,
                green: 0,
            }
        );
        assert_eq!(
            ColorCount::from_str("4 red, 2 green").unwrap(),
            ColorCount {
                red: 4,
                green: 2,
                blue: 0,
            }
        );
        assert_eq!(
            ColorCount::from_str("6 blue, 2 green").unwrap(),
            ColorCount {
                blue: 6,
                green: 2,
                red: 0,
            }
        );
    }

    #[test]
    fn testing_possiblities() {
        let max_counts = ColorCount {
            red: 12,
            green: 13,
            blue: 14,
        };

        assert_eq!(max_counts.is_possible(&ColorCount::_new(4, 0, 3)), true);
        assert_eq!(max_counts.is_possible(&ColorCount::_new(1, 2, 6)), true);
        assert_eq!(max_counts.is_possible(&ColorCount::_new(0, 2, 0)), true);

        assert_eq!(max_counts.is_possible(&ColorCount::_new(20, 8, 6)), false);
        assert_eq!(max_counts.is_possible(&ColorCount::_new(4, 13, 5)), true);
        assert_eq!(max_counts.is_possible(&ColorCount::_new(1, 5, 0)), true);
    }

    #[test]
    fn calculate_power() {
        assert_eq!(
            ColorCount::calculate_power(vec![
                ColorCount::_new(4, 0, 3),
                ColorCount::_new(1, 2, 6),
                ColorCount::_new(0, 2, 0),
            ]),
            48
        );
    }

    #[test]
    fn retrieving_the_game_id() {
        assert_eq!(
            get_game_id("Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green"),
            Some(1)
        );
        assert_eq!(
            get_game_id("Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red"),
            Some(3)
        );
        assert_eq!(get_game_id("Some random string"), None);
    }

    #[test]
    fn parse_game_results() {
        assert_eq!(
            Game::from_str("Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green").unwrap(),
            Game {
                id: 1,
                results: vec![
                    ColorCount::_new(4, 0, 3),
                    ColorCount::_new(1, 2, 6),
                    ColorCount::_new(0, 2, 0),
                ],
            }
        );
        assert_eq!(
            Game::from_str(
                "Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red"
            )
            .unwrap(),
            Game {
                id: 3,
                results: vec![
                    ColorCount::_new(20, 8, 6),
                    ColorCount::_new(4, 13, 5),
                    ColorCount::_new(1, 5, 0),
                ],
            }
        );
    }

    #[test]
    fn part_one_correct() {
        assert_eq!(
            part_one(
                "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green"
            ),
            8
        );
    }

    #[test]
    fn part_two_correct() {
        assert_eq!(
            part_two(
                "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green"
            ),
            2286
        );
    }
}
