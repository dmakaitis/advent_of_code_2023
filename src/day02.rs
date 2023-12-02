struct Result {
    red: u32,
    green: u32,
    blue: u32,
}

impl Result {
    fn parse(text: &str) -> Result {
        let mut red = 0;
        let mut green = 0;
        let mut blue = 0;

        let parts = text.split(',');
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
                _ => {}
            }
        }

        Result { red, green, blue }
    }

    fn is_possible(&self, other: &Result) -> bool {
        other.red <= self.red && other.green <= self.green && other.blue <= self.blue
    }

    fn calculate_power(results: Vec<Result>) -> u32 {
        let mut max_results = Result { red: 0, green: 0, blue: 0 };

        for result in results {
            if result.red > max_results.red {
                max_results.red = result.red;
            }
            if result.green > max_results.green {
                max_results.green = result.green;
            }
            if result.blue > max_results.blue {
                max_results.blue = result.blue;
            }
        }

        max_results.red * max_results.green * max_results.blue
    }
}

fn get_game_id(text: &str) -> Option<u32> {
    let text = text.strip_prefix("Game ")?;
    let index = text.find(':')?;
    text[..index].parse().ok()
}

struct Game {
    id: u32,
    results: Vec<Result>,
}

impl Game {
    fn from(text: &str) -> Game {
        let id = get_game_id(text).unwrap();

        let text = text.strip_prefix(&format!("Game {id}: ")).unwrap();

        let results = text.split(';');
        let results: Vec<_> = results
            .map(Result::parse)
            .collect();

        Game { id, results }
    }
}

pub fn part_one(input: &str) -> u32 {
    let max_cubes = Result { red: 12, green: 13, blue: 14 };

    input.lines()
        .map(Game::from)
        .filter(|game| game.results.iter()
            .all(|result| max_cubes.is_possible(result))
        )
        .map(|game| game.id)
        .sum()
}

pub fn part_two(input: &str) -> u32 {
    input.lines()
        .map(Game::from)
        .map(|game| Result::calculate_power(game.results))
        .sum()
}

#[cfg(test)]
mod tests {
    use crate::day02::*;

    #[test]
    fn part_one_correct() {
        assert_eq!(part_one("Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green"), 8);
    }

    #[test]
    fn part_two_correct() {
        assert_eq!(part_two("Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green"), 2286);
    }
}
