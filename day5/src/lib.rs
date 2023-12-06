use env_logger;
use log;
use nom::{
    bytes::complete::tag,
    character::complete::{alphanumeric1, char, line_ending, space1, u64 as parse_u64},
    combinator::map,
    multi::separated_list1,
    sequence::{pair, preceded, separated_pair, terminated},
    IResult,
};
use std::fmt::Formatter;
use std::fmt;
use std::fmt::Debug;
mod debug;

#[derive(PartialEq, Clone, Copy)]
struct NumberRange {
    start: u64,
    end: u64,
}

impl NumberRange {
    fn contains(&self, number: &u64) -> bool {
        *number >= self.start && *number <= self.end
    }

    fn overlap(&self, other: &NumberRange) -> Option<NumberRange> {
        if self.start <= other.end && self.end >= other.start {
            Some(NumberRange {
                start: self.start.max(other.start),
                end: self.end.min(other.end),
            })
        } else {
            None
        }
    }

    fn difference(&self, other: &NumberRange) -> Vec<NumberRange> {
        let mut difference = Vec::new();
        if self.start < other.start {
            difference.push(NumberRange {
                start: self.start,
                end: self.end.min(other.start - 1),
            });
        }
        if self.end > other.end {
            difference.push(NumberRange {
                start: self.start.max(other.end + 1),
                end: self.end,
            });
        }
        difference
    }
}

impl Debug for NumberRange {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "({}; {})", self.start, self.end)
    }
}

#[derive(PartialEq, Clone, Copy)]
struct NumberMap {
    pub src: NumberRange,
    pub dest: NumberRange,
}

impl NumberMap {
    fn parse(input: &str) -> IResult<&str, Self> {
        let parse_three_numbers = separated_pair(
            separated_pair(parse_u64, space1, parse_u64),
            space1,
            parse_u64,
        );

        map(parse_three_numbers, |((dest_start, src_start), length)| {
            NumberMap {
                src: NumberRange {
                    start: src_start,
                    end: src_start + length - 1,
                },
                dest: NumberRange {
                    start: dest_start,
                    end: dest_start + length - 1,
                },
            }
        })(input)
    }

    fn map(&self, number: &u64) -> Option<u64> {
        if self.src.contains(number) {
            Some(self.dest.start + (number - self.src.start))
        } else {
            None
        }
    }

    fn map_reverse(&self, number: &u64) -> Option<u64> {
        if self.dest.contains(number) {
            Some(self.src.start + (number - self.dest.start))
        } else {
            None
        }
    }

    fn map_range(&self, number_range: &NumberRange) -> Option<NumberRange> {
        if let Some(overlap) = self.src.overlap(number_range) {
            Some(NumberRange {
                start: self.dest.start + (overlap.start - self.src.start),
                end: self.dest.start + (overlap.end - self.src.start),
            })
        } else {
            return None;
        }
    }

    fn map_range_reverse(&self, number_range: &NumberRange) -> Option<NumberRange> {
        if let Some(overlap) = self.dest.overlap(number_range) {
            Some(NumberRange {
                start: self.src.start + (overlap.start - self.dest.start),
                end: self.src.start + (overlap.end - self.dest.start),
            })
        } else {
            return None;
        }
    }

    fn chain(before: &NumberMap, after: &NumberMap) -> Option<NumberMap> {
        if let Some(overlap) = before.dest.overlap(&after.src) {
            Some(NumberMap {
                src: before.map_range_reverse(&overlap).unwrap(),
                dest: after.map_range(&overlap).unwrap(),
            })
        } else {
            None
        }
    }

    fn split_by_non_chainable_dest(before: &NumberMap, after: &NumberMap) -> Vec<NumberMap> {
        before
            .dest
            .difference(&after.src)
            .iter()
            .map(|unmapped_dest| NumberMap {
                src: before.map_range_reverse(&unmapped_dest).unwrap(),
                dest: unmapped_dest.clone(),
            })
            .collect()
    }
}
impl Debug for NumberMap {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{:?} --> {:?}", self.src, self.dest)
    }
}

#[derive(Debug, PartialEq, Clone)]
struct MultiMap {
    number_maps: Vec<NumberMap>,
}

impl MultiMap {
    // seed-to-soil map:
    // 50 98 2
    // 52 50 48
    fn parse(input: &str) -> IResult<&str, Self> {
        // parse title (i.e. "seed-to-soil map:")
        let parse_title_name = map(
            separated_pair(
                separated_pair(alphanumeric1, char('-'), alphanumeric1),
                char('-'),
                alphanumeric1,
            ),
            |((a, b), c)| format!("{a}-{b}-{c}"),
        );
        let parse_title = pair(parse_title_name, tag(" map:"));

        // parse list of ranges
        let parse_ranges = separated_list1(line_ending, NumberMap::parse);

        let parse_map = preceded(terminated(parse_title, line_ending), parse_ranges);

        map(parse_map, |ranges| MultiMap {
            number_maps: ranges,
        })(input)
    }

    fn map(&self, number: &u64) -> u64 {
        for range in &self.number_maps {
            if let Some(mapped_number) = range.map(number) {
                return mapped_number;
            }
        }
        number.clone()
    }

    fn map_range(&self, number_range: &NumberRange) -> Vec<NumberRange> {
        let mut mapped_ranges = Vec::new();
        let mut unmapped_ranges = vec![number_range.clone()];

        for number_map in &self.number_maps {
            let mut new_unmapped_ranges = Vec::new();
            for unmapped_range in unmapped_ranges {
                if let Some(mapped_range) = number_map.map_range(&unmapped_range) {
                    mapped_ranges.push(mapped_range);
                }

                new_unmapped_ranges.append(&mut unmapped_range.difference(&number_map.src));
            }
            unmapped_ranges = new_unmapped_ranges;
        }

        mapped_ranges.append(&mut unmapped_ranges);
        mapped_ranges
    }

    fn chain_number_map(before: &NumberMap, after: &MultiMap) -> Vec<NumberMap> {
        let mut chained_maps = Vec::new();
        let mut unmapped_maps = vec![before.clone()];

        for after_map in &after.number_maps {
            let mut new_unmapped_maps = Vec::new();
            for before_map in unmapped_maps {
                if let Some(chained_map) = NumberMap::chain(&before_map, &after_map) {
                    chained_maps.push(chained_map);
                }

                new_unmapped_maps.append(&mut NumberMap::split_by_non_chainable_dest(
                    &before_map,
                    &after_map,
                ));
            }
            unmapped_maps = new_unmapped_maps;
        }

        chained_maps.append(&mut unmapped_maps);
        chained_maps
    }

    fn chain(before: &MultiMap, after: &MultiMap) -> MultiMap {
        // for each before number map chain it with after and collect the results in a flat list
        before.number_maps.iter().fold(
            MultiMap {
                number_maps: Vec::new(),
            },
            |mut acc, before_map| {
                let mut chained_map = MultiMap::chain_number_map(before_map, &after);
                acc.number_maps.append(&mut chained_map);
                acc
            },
        )
    }
}

#[derive(Debug, PartialEq, Clone)]
struct Almanac {
    seeds: Vec<u64>,
    maps: Vec<MultiMap>,
}

impl Almanac {
    fn parse(input: &str) -> IResult<&str, Self> {
        let parse_seeds = preceded(tag("seeds: "), separated_list1(space1, parse_u64));

        let empty_line = |input| pair(line_ending, line_ending)(input);

        let parse_maps = separated_list1(empty_line, MultiMap::parse);

        let parse_almanac = separated_pair(parse_seeds, empty_line, parse_maps);

        map(parse_almanac, |(seeds, maps)| Almanac { seeds, maps })(input)
    }

    fn map(&self, number: u64) -> u64 {
        let mut mapped_number = number;
        log::debug!("{}:", number);

        for map in &self.maps {
            let new_mapped_number = map.map(&mapped_number);

            if mapped_number != new_mapped_number {
                log::debug!("=> {}", mapped_number);
            } else {
                log::debug!("== {}", mapped_number);
            }

            mapped_number = new_mapped_number;
        }
        mapped_number
    }

    fn get_min_location_number(&self) -> u64 {
        self.seeds.iter().map(|seed| self.map(*seed)).min().unwrap()
    }
}

#[derive(Debug, PartialEq, Clone)]
struct Ranged_Almanac {
    seeds: Vec<NumberRange>,
    maps: Vec<MultiMap>,
}

impl Ranged_Almanac {
    fn parse(input: &str) -> IResult<&str, Self> {
        let parse_seed_range = map(
            separated_pair(parse_u64, space1, parse_u64),
            |(start, length)| NumberRange {
                start,
                end: start + length - 1,
            },
        );

        let parse_seeds = preceded(tag("seeds: "), separated_list1(space1, parse_seed_range));

        let empty_line = |input| pair(line_ending, line_ending)(input);

        let parse_maps = separated_list1(empty_line, MultiMap::parse);

        let parse_almanac = separated_pair(parse_seeds, empty_line, parse_maps);

        map(parse_almanac, |(seeds, maps)| Ranged_Almanac {
            seeds,
            maps,
        })(input)
    }

    fn get_min_location_number(&self) -> u64 {
        // chain all maps together
        let chained_map = self.maps.iter().cloned().reduce(|before, after| MultiMap::chain(&before, &after)).unwrap();

        chained_map.number_maps.iter().for_each(|f| println!("{:?}", f));

        let mapped_ranges = self.seeds.iter().map(|seed| chained_map.map_range(seed)).flatten().collect::<Vec<NumberRange>>();
        
        println!("mapped ranges:");
        mapped_ranges.iter().for_each(|f| println!("{:?}", f));

        mapped_ranges.iter().map(|range| range.start).min().unwrap()
    }
}

fn get_min_location_number(input: &str) -> u64 {
    let (_, almanac) = Almanac::parse(input).unwrap();
    almanac.get_min_location_number()
}

fn get_min_location_number_ranged(input: &str) -> u64 {
    let (_, almanac) = Ranged_Almanac::parse(input).unwrap();
    almanac.get_min_location_number()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_number_map_parse() {
        let input = "10 20 30";
        let expected = NumberMap {
            src: NumberRange { start: 20, end: 49 },
            dest: NumberRange { start: 10, end: 39 },
        };
        let result = NumberMap::parse(input).unwrap().1;
        assert_eq!(result, expected);
    }

    #[test]
    fn test_multi_map_parse() {
        let input = "seed-to-soil map:\n50 98 2\n52 50 48";
        let expected = MultiMap {
            number_maps: vec![
                NumberMap {
                    src: NumberRange { start: 98, end: 99 },
                    dest: NumberRange { start: 50, end: 51 },
                },
                NumberMap {
                    src: NumberRange { start: 50, end: 97 },
                    dest: NumberRange { start: 52, end: 99 },
                },
            ],
        };
        let result = MultiMap::parse(input).unwrap().1;
        assert_eq!(result, expected);
    }

    #[test]
    fn test_almanac_parse() {
        let input = "seeds: 79 14 55 13

seed-to-soil map:
50 98 2
52 50 48

soil-to-fertilizer map:
0 15 37
37 52 2
39 0 15
";

        let expected = Almanac {
            seeds: vec![79, 14, 55, 13],
            maps: vec![
                MultiMap {
                    number_maps: vec![
                        NumberMap {
                            src: NumberRange { start: 98, end: 99 },
                            dest: NumberRange { start: 50, end: 51 },
                        },
                        NumberMap {
                            src: NumberRange { start: 50, end: 97 },
                            dest: NumberRange { start: 52, end: 99 },
                        },
                    ],
                },
                MultiMap {
                    number_maps: vec![
                        NumberMap {
                            src: NumberRange { start: 15, end: 51 },
                            dest: NumberRange { start: 0, end: 36 },
                        },
                        NumberMap {
                            src: NumberRange { start: 52, end: 53 },
                            dest: NumberRange { start: 37, end: 38 },
                        },
                        NumberMap {
                            src: NumberRange { start: 0, end: 14 },
                            dest: NumberRange { start: 39, end: 53 },
                        },
                    ],
                },
            ],
        };
        let result = Almanac::parse(input).unwrap().1;
        assert_eq!(result, expected);
    }

    #[test]
    fn test_ranged_almanac_parse() {
        let input = "seeds: 79 14 55 13

seed-to-soil map:
50 98 2
52 50 48

soil-to-fertilizer map:
0 15 37
37 52 2
39 0 15
";

        let expected = Ranged_Almanac {
            seeds: vec![
                NumberRange { start: 79, end: 92 },
                NumberRange { start: 55, end: 67 },
            ],
            maps: vec![
                MultiMap {
                    number_maps: vec![
                        NumberMap {
                            src: NumberRange { start: 98, end: 99 },
                            dest: NumberRange { start: 50, end: 51 },
                        },
                        NumberMap {
                            src: NumberRange { start: 50, end: 97 },
                            dest: NumberRange { start: 52, end: 99 },
                        },
                    ],
                },
                MultiMap {
                    number_maps: vec![
                        NumberMap {
                            src: NumberRange { start: 15, end: 51 },
                            dest: NumberRange { start: 0, end: 36 },
                        },
                        NumberMap {
                            src: NumberRange { start: 52, end: 53 },
                            dest: NumberRange { start: 37, end: 38 },
                        },
                        NumberMap {
                            src: NumberRange { start: 0, end: 14 },
                            dest: NumberRange { start: 39, end: 53 },
                        },
                    ],
                },
            ],
        };
        let result = Ranged_Almanac::parse(input).unwrap().1;
        assert_eq!(result, expected);
    }

    #[test]
    fn test_range_contains() {
        let range = NumberRange { start: 10, end: 20 };
        assert!(range.contains(&10));
        assert!(range.contains(&15));
        assert!(range.contains(&20));
        assert!(!range.contains(&21));
        assert!(!range.contains(&9));
    }

    #[test]
    fn test_range_overlap() {
        let range = NumberRange { start: 10, end: 20 };
        let other = NumberRange { start: 15, end: 25 };
        let expected = NumberRange { start: 15, end: 20 };
        let result = range.overlap(&other).unwrap();
        assert_eq!(result, expected);
    }

    #[test]
    fn test_range_difference() {
        let range = NumberRange { start: 10, end: 20 };
        let other = NumberRange { start: 15, end: 25 };
        let expected = vec![NumberRange { start: 10, end: 14 }];
        let result = range.difference(&other);
        assert_eq!(result, expected);
    }

    #[test]
    fn test_number_map_map_range() {
        let number_map = NumberMap {
            src: NumberRange { start: 10, end: 20 },
            dest: NumberRange { start: 50, end: 60 },
        };
        let number_range = NumberRange { start: 15, end: 25 };
        let expected = NumberRange { start: 55, end: 60 };
        let result = number_map.map_range(&number_range).unwrap();
        assert_eq!(result, expected);
    }

    #[test]
    fn test_multi_map_map_range() {
        let multi_map = MultiMap {
            number_maps: vec![
                NumberMap {
                    src: NumberRange { start: 10, end: 20 },
                    dest: NumberRange { start: 50, end: 60 },
                },
                NumberMap {
                    src: NumberRange { start: 25, end: 30 },
                    dest: NumberRange { start: 75, end: 80 },
                },
            ],
        };
        let number_range = NumberRange { start: 15, end: 30 };
        let expected = vec![
            NumberRange { start: 55, end: 60 },
            NumberRange { start: 75, end: 80 },
            NumberRange { start: 21, end: 24 },
        ];
        let result = multi_map.map_range(&number_range);
        assert_eq!(result, expected);
    }

    #[test]
    fn small_input() {
        env_logger::init();

        // The easiest way to open the data is to include it into the generated binary.
        let input = include_str!("../input/small.txt");
        assert_eq!(get_min_location_number(input), 35)
    }

    #[test]
    fn large_input() {
        // You can also read the file completely into memory
        let file = std::fs::read_to_string("input/big.txt").expect("Could not open input file");
        assert_eq!(get_min_location_number(&file), 289863851)
    }

    #[test]
    fn small_input_ranged() {
        // The easiest way to open the data is to include it into the generated binary.
        let input = include_str!("../input/small.txt");
        assert_eq!(get_min_location_number_ranged(input), 46)
    }

    #[test]
    fn large_input_ranged() {
        // You can also read the file completely into memory
        let file = std::fs::read_to_string("input/big.txt").expect("Could not open input file");
        assert_eq!(get_min_location_number_ranged(&file), 60568880)
    }
}
