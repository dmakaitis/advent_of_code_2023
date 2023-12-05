use std::str::FromStr;

#[derive(Debug, Clone, PartialEq, Eq)]
struct ParseError {
    msg: &'static str,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct SeedList {
    seeds: Vec<i64>,
}

impl FromStr for SeedList {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if let Some(seeds) = s.strip_prefix("seeds: ") {
            let result: Result<Vec<_>, _> =
                seeds.split_whitespace().map(|s| s.parse::<i64>()).collect();

            Ok(SeedList {
                seeds: result.map_err(|_| ParseError {
                    msg: "Invalid seed number",
                })?,
            })
        } else {
            Err(ParseError {
                msg: "Not implemented",
            })
        }
    }
}

//////////////////////////////////////////////////////////////////////////////////////////////
//////////////////////////////////////////////////////////////////////////////////////////////
//////////////////////////////////////////////////////////////////////////////////////////////

#[derive(Debug, Clone, PartialEq, Eq)]
struct PartialMapper {
    start: i64,
    end: i64,
    diff: i64,
}

impl FromStr for PartialMapper {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut parts = s.split_whitespace();

        let min_output = parts.next().ok_or(ParseError {
            msg: "Missing first range component ",
        })?;
        let min_output = min_output.parse::<i64>().map_err(|_| ParseError {
            msg: "Failed to parse output component",
        })?;

        let min_input = parts.next().ok_or(ParseError {
            msg: "Missing first range component ",
        })?;
        let min_input = min_input.parse::<i64>().map_err(|_| ParseError {
            msg: "Failed to parse input component",
        })?;

        let range = parts.next().ok_or(ParseError {
            msg: "Missing first range component ",
        })?;
        let range = range.parse::<i64>().map_err(|_| ParseError {
            msg: "Failed to parse range component",
        })?;

        Ok(PartialMapper {
            start: min_input,
            end: min_input + range,
            diff: min_output - min_input,
        })
    }
}

impl PartialMapper {
    fn eval(&self, input: i64) -> Option<i64> {
        if input < self.start || input >= self.end {
            None
        } else {
            Some(input + self.diff)
        }
    }
}

//////////////////////////////////////////////////////////////////////////////////////////////
//////////////////////////////////////////////////////////////////////////////////////////////
//////////////////////////////////////////////////////////////////////////////////////////////

#[derive(Debug, PartialEq, Eq)]
struct Mapper {
    name: String,
    partials: Vec<PartialMapper>,
}

impl FromStr for Mapper {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut lines = s.lines();

        let name = lines.next().ok_or(ParseError {
            msg: "No mapper name ",
        })?;
        let name = name.strip_suffix(" map:").ok_or(ParseError {
            msg: "Missing trailing colon after mapper name",
        })?;
        let name = String::from_str(name).unwrap();

        let partials: Result<Vec<_>, _> = lines.map(PartialMapper::from_str).collect();
        let mut partials = partials?;

        partials.sort_by(|a, b| a.start.cmp(&b.start));

        Ok(Mapper { name, partials })
    }
}

impl Mapper {
    fn eval(&self, input: i64) -> i64 {
        self.partials
            .iter()
            .filter_map(|p| p.eval(input))
            .next()
            .unwrap_or(input)
    }

    fn eval_ranges(&self, input: &Vec<(i64, i64)>) -> Vec<(i64, i64)> {
        let mut result: Vec<(i64, i64)> = vec![];

        for &(start, end) in input {
            let mut i = start;
            for m in &self.partials {
                if i < m.start {
                    if end <= m.start {
                        // The entire input range is before the first partial, so remains unchanged:
                        result.push((i, end));
                        i = end;
                        break;
                    }

                    // Push the portion if the input range that is before the first partial:
                    result.push((i, m.start));
                    i = m.start;
                }

                if i < m.end {
                    if end <= m.end {
                        // The rest of the input range is before the end of the partial, so
                        // calculate and push the resulting range:
                        result.push((i + m.diff, end + m.diff));
                        i = end;
                        break;
                    }

                    // Push the rest of the partial range into the results and update pointers to
                    // start looking for the next partial:
                    result.push((i + m.diff, m.end + m.diff));
                    i = m.end;
                }
            }

            if i < end {
                // Push any part of the input range that lies beyond the last partial:
                result.push((i, end));
            }
        }

        // Sort the results since applying the partials probably resulted in our output ranges being
        // out of order:
        result.sort_by(|(a, _), (b, _)| a.cmp(b));

        result
    }
}

//////////////////////////////////////////////////////////////////////////////////////////////
//////////////////////////////////////////////////////////////////////////////////////////////
//////////////////////////////////////////////////////////////////////////////////////////////

///
///
/// #Argument
///
/// 'input' - The input.
pub fn part_one(input: &str) -> i64 {
    let mut parts = input.split("\n\n");

    let seeds: SeedList = parts.next().unwrap().parse().unwrap();

    let seed_to_soil_map: Mapper = parts.next().unwrap().parse().unwrap();
    let soil_to_fertilizer_map: Mapper = parts.next().unwrap().parse().unwrap();
    let fertilizer_to_water_map: Mapper = parts.next().unwrap().parse().unwrap();
    let water_to_light_map: Mapper = parts.next().unwrap().parse().unwrap();
    let light_to_temperature_map: Mapper = parts.next().unwrap().parse().unwrap();
    let temperature_to_humidity_map: Mapper = parts.next().unwrap().parse().unwrap();
    let humidity_to_location_map: Mapper = parts.next().unwrap().parse().unwrap();

    seeds
        .seeds
        .iter()
        .map(|s| seed_to_soil_map.eval(*s))
        .map(|s| soil_to_fertilizer_map.eval(s))
        .map(|s| fertilizer_to_water_map.eval(s))
        .map(|s| water_to_light_map.eval(s))
        .map(|s| light_to_temperature_map.eval(s))
        .map(|s| temperature_to_humidity_map.eval(s))
        .map(|s| humidity_to_location_map.eval(s))
        .min()
        .unwrap()
}

///
///
/// #Argument
///
/// 'input' - The input.
pub fn part_two(input: &str) -> i64 {
    let mut parts = input.split("\n\n");

    let seeds: SeedList = parts.next().unwrap().parse().unwrap();

    let seed_to_soil_map: Mapper = parts.next().unwrap().parse().unwrap();
    let soil_to_fertilizer_map: Mapper = parts.next().unwrap().parse().unwrap();
    let fertilizer_to_water_map: Mapper = parts.next().unwrap().parse().unwrap();
    let water_to_light_map: Mapper = parts.next().unwrap().parse().unwrap();
    let light_to_temperature_map: Mapper = parts.next().unwrap().parse().unwrap();
    let temperature_to_humidity_map: Mapper = parts.next().unwrap().parse().unwrap();
    let humidity_to_location_map: Mapper = parts.next().unwrap().parse().unwrap();

    let mut seeds: Vec<_> = seeds
        .seeds
        .chunks(2)
        .map(|chunk| (chunk[0], chunk[0] + chunk[1]))
        .collect();

    // Make sure our input ranges are in order:
    seeds.sort_by(|(a, _), (b, _)| a.cmp(b));

    // Apply each of the mappers in sequence to the input ranges:
    let seeds = seed_to_soil_map.eval_ranges(&seeds);
    let seeds = soil_to_fertilizer_map.eval_ranges(&seeds);
    let seeds = fertilizer_to_water_map.eval_ranges(&seeds);
    let seeds = water_to_light_map.eval_ranges(&seeds);
    let seeds = light_to_temperature_map.eval_ranges(&seeds);
    let seeds = temperature_to_humidity_map.eval_ranges(&seeds);
    let seeds = humidity_to_location_map.eval_ranges(&seeds);

    // Since the output ranges are sorted, the minimum value is simply the start of the first
    // output range:
    let (start, _) = seeds[0];

    start
}

#[cfg(test)]
mod tests {
    use crate::day05::*;

    #[test]
    fn parse_seed_list() {
        assert_eq!(
            SeedList::from_str("seeds: 79 14 55 13"),
            Ok(SeedList {
                seeds: vec![79, 14, 55, 13]
            })
        );

        assert!(SeedList::from_str("blah").is_err());
        assert!(SeedList::from_str("seeds: 79 14 blah 13").is_err());
    }

    #[test]
    fn parse_partial_maps() {
        assert!(PartialMapper::from_str("50 98 2").is_ok());
        assert!(PartialMapper::from_str("blah").is_err());
        assert!(PartialMapper::from_str("50 blah 2").is_err());
        assert!(PartialMapper::from_str("50 2").is_err());
    }

    #[test]
    fn evaluate_partial_map() {
        let map = PartialMapper::from_str("50 98 2").unwrap();

        assert_eq!(map.eval(97), None);
        assert_eq!(map.eval(98), Some(50));
        assert_eq!(map.eval(99), Some(51));
        assert_eq!(map.eval(100), None);
    }

    #[test]
    fn parse_mappers() {
        assert!(Mapper::from_str(
            "seed-to-soil map:
50 98 2
52 50 48"
        )
        .is_ok());

        assert!(Mapper::from_str("blah").is_err());
    }

    #[test]
    fn evaluate_mapper() {
        let mapper = Mapper::from_str(
            "seed-to-soil map:
50 98 2
52 50 48",
        )
        .unwrap();

        assert_eq!(mapper.eval(0), 0);
        assert_eq!(mapper.eval(1), 1);

        assert_eq!(mapper.eval(48), 48);
        assert_eq!(mapper.eval(49), 49);
        assert_eq!(mapper.eval(50), 52);
        assert_eq!(mapper.eval(51), 53);

        assert_eq!(mapper.eval(96), 98);
        assert_eq!(mapper.eval(97), 99);
        assert_eq!(mapper.eval(98), 50);
        assert_eq!(mapper.eval(99), 51);
    }

    #[test]
    fn eval_ranges() {
        let mapper = Mapper::from_str(
            "seed-to-soil map:
50 98 2
52 50 48",
        )
        .unwrap();

        assert_eq!(mapper.eval_ranges(&vec![]), vec![]);
        assert_eq!(mapper.eval_ranges(&vec![(0, 5)]), vec![(0, 5)]);
        assert_eq!(mapper.eval_ranges(&vec![(98, 99)]), vec![(50, 51)]);
    }

    #[test]
    fn part_one_correct() {
        assert_eq!(
            part_one(
                "seeds: 79 14 55 13

seed-to-soil map:
50 98 2
52 50 48

soil-to-fertilizer map:
0 15 37
37 52 2
39 0 15

fertilizer-to-water map:
49 53 8
0 11 42
42 0 7
57 7 4

water-to-light map:
88 18 7
18 25 70

light-to-temperature map:
45 77 23
81 45 19
68 64 13

temperature-to-humidity map:
0 69 1
1 0 69

humidity-to-location map:
60 56 37
56 93 4"
            ),
            35
        );
    }

    #[test]
    fn part_two_correct() {
        assert_eq!(
            part_two(
                "seeds: 79 14 55 13

seed-to-soil map:
50 98 2
52 50 48

soil-to-fertilizer map:
0 15 37
37 52 2
39 0 15

fertilizer-to-water map:
49 53 8
0 11 42
42 0 7
57 7 4

water-to-light map:
88 18 7
18 25 70

light-to-temperature map:
45 77 23
81 45 19
68 64 13

temperature-to-humidity map:
0 69 1
1 0 69

humidity-to-location map:
60 56 37
56 93 4"
            ),
            46
        );
    }
}
