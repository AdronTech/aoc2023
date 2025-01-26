use memoize::memoize;
use nom::{
    character::complete::{char, one_of, space1, u128 as parse_u128},
    combinator::map,
    multi::{many1, separated_list1},
    sequence::separated_pair,
    IResult,
};

mod debug;

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
pub enum Status {
    Operational,
    Damaged,
    Unknown,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
pub enum StatusRange {
    ExactlyDamaged(u128),
    OneOrMoreOperational,
    ZeroOrMoreOperational,
}

fn parse_individual_spring(input: &str) -> IResult<&str, Status> {
    map(one_of("#?."), |c| match c {
        '#' => Status::Damaged,
        '?' => Status::Unknown,
        '.' => Status::Operational,
        _ => panic!("Invalid input"),
    })(input)
}

fn parse_row_individual(input: &str) -> IResult<&str, Vec<Status>> {
    many1(parse_individual_spring)(input)
}

fn enrich_row_range(row_range: Vec<StatusRange>) -> Vec<StatusRange> {
    // add zero or more operational in front
    let mut enriched = vec![];
    enriched.push(StatusRange::ZeroOrMoreOperational);
    row_range.iter().for_each(|r| {
        enriched.push(*r);
        enriched.push(StatusRange::OneOrMoreOperational)
    });
    enriched.pop(); // remove last one or more operational

    // add zero or more operational at the end
    enriched.push(StatusRange::ZeroOrMoreOperational);
    enriched
}

fn parse_row_range(input: &str) -> IResult<&str, Vec<StatusRange>> {
    separated_list1(
        char(','),
        map(parse_u128, |n| StatusRange::ExactlyDamaged(n)),
    )(input)
}

fn parse_row_report(input: &str) -> IResult<&str, (Vec<Status>, Vec<StatusRange>)> {
    separated_pair(parse_row_individual, space1, parse_row_range)(input)
}

fn calc_combinations_rec_with_status(
    row_prefix: &Vec<Status>,
    row: &Vec<Status>,
    row_range: &Vec<StatusRange>,
    status: &Status,
) -> u128 {
    let mut row = row.clone();
    row[0] = status.clone();

    calc_combinations_rec(&row_prefix, &row, row_range)
}

fn calc_combinations_rec(
    row_prefix: &Vec<Status>,
    row: &Vec<Status>,
    row_range: &Vec<StatusRange>,
) -> u128 {
    if row.len() == 0 {
        if row_range.len() <= 1 {
            // println!("Valid!");
            return 1;
        } else {
            return 0;
        }
    }

    // debug::print_report(&row_prefix, &row, &row_range);
    // println!();
    // println!("-------------------");

    let first_spring = row[0];
    let cur_row_range = &row_range[0];

    let mut new_prefix = row_prefix.clone();
    new_prefix.push(first_spring);

    match first_spring {
        Status::Damaged => {
            match cur_row_range {
                StatusRange::OneOrMoreOperational => {
                    return 0; // we expect at least one operational is required
                }
                StatusRange::ExactlyDamaged(n) => {
                    let mut new_row_range = row_range.clone();
                    if *n == 1 {
                        new_row_range.remove(0);
                    } else {
                        new_row_range[0] = StatusRange::ExactlyDamaged(n - 1);
                    }
                    return calc_combinations_rec(&new_prefix, &row[1..].to_vec(), &new_row_range);
                }
                StatusRange::ZeroOrMoreOperational => {
                    if row_range.len() <= 1 {
                        return 0; // there should be another expected range, if there is not then the row is not possible
                    }

                    let mut new_row_range = row_range.clone();
                    new_row_range.remove(0);

                    let cur_row_range = &new_row_range[0];
                    match cur_row_range {
                        StatusRange::ExactlyDamaged(n) => {
                            if *n == 1 {
                                new_row_range.remove(0);
                            } else {
                                new_row_range[0] = StatusRange::ExactlyDamaged(n - 1);
                            }
                        }
                        _ => panic!("This should not happen"),
                    };
                    return calc_combinations_rec(&new_prefix, &row[1..].to_vec(), &new_row_range);
                }
            }
        }
        Status::Operational => {
            match cur_row_range {
                StatusRange::ExactlyDamaged(_) => {
                    return 0; // we expect a damaged spring here
                }
                StatusRange::OneOrMoreOperational => {
                    let mut new_row_range = row_range.clone();
                    new_row_range[0] = StatusRange::ZeroOrMoreOperational;
                    return calc_combinations_rec(&new_prefix, &row[1..].to_vec(), &new_row_range);
                }
                StatusRange::ZeroOrMoreOperational => {
                    return calc_combinations_rec(&new_prefix, &row[1..].to_vec(), row_range)
                }
            }
        }
        Status::Unknown => {
            return calc_combinations_rec_with_status(
                &row_prefix,
                &row,
                row_range,
                &Status::Operational,
            ) + calc_combinations_rec_with_status(
                &row_prefix,
                &row,
                row_range,
                &Status::Damaged,
            );
        }
    }
}

#[memoize]
fn calc_combinations_rec_fast_with_status(
    row: Vec<Status>,
    row_range: Vec<StatusRange>,
    status: Status,
) -> u128 {
    let mut row = row.clone();
    row[0] = status.clone();

    calc_combinations_rec_fast(row, row_range)
}

#[memoize]
fn calc_combinations_rec_fast(row: Vec<Status>, row_range: Vec<StatusRange>) -> u128 {
    if row.len() == 0 {
        if row_range.len() <= 1 {
            // println!("Valid!");
            return 1;
        } else {
            return 0;
        }
    }

    // debug::print_report(&row_prefix, &row, &row_range);
    // println!();
    // println!("-------------------");

    let first_spring = row[0];
    let cur_row_range = &row_range[0];

    match first_spring {
        Status::Damaged => {
            match cur_row_range {
                StatusRange::OneOrMoreOperational => {
                    return 0; // we expect at least one operational is required
                }
                StatusRange::ExactlyDamaged(n) => {
                    let mut new_row_range = row_range.clone();
                    if *n == 1 {
                        new_row_range.remove(0);
                    } else {
                        new_row_range[0] = StatusRange::ExactlyDamaged(n - 1);
                    }
                    return calc_combinations_rec_fast(row[1..].to_vec(), new_row_range);
                }
                StatusRange::ZeroOrMoreOperational => {
                    if row_range.len() <= 1 {
                        return 0; // there should be another expected range, if there is not then the row is not possible
                    }

                    let mut new_row_range = row_range.clone();
                    new_row_range.remove(0);

                    let cur_row_range = &new_row_range[0];
                    match cur_row_range {
                        StatusRange::ExactlyDamaged(n) => {
                            if *n == 1 {
                                new_row_range.remove(0);
                            } else {
                                new_row_range[0] = StatusRange::ExactlyDamaged(n - 1);
                            }
                        }
                        _ => panic!("This should not happen"),
                    };
                    return calc_combinations_rec_fast(row[1..].to_vec(), new_row_range);
                }
            }
        }
        Status::Operational => {
            match cur_row_range {
                StatusRange::ExactlyDamaged(_) => {
                    return 0; // we expect a damaged spring here
                }
                StatusRange::OneOrMoreOperational => {
                    let mut new_row_range = row_range.clone();
                    new_row_range[0] = StatusRange::ZeroOrMoreOperational;
                    return calc_combinations_rec_fast(row[1..].to_vec(), new_row_range);
                }
                StatusRange::ZeroOrMoreOperational => {
                    return calc_combinations_rec_fast(row[1..].to_vec(), row_range)
                }
            }
        }
        Status::Unknown => {
            return calc_combinations_rec_fast_with_status(row.clone(), row_range.clone(), Status::Operational)
                + calc_combinations_rec_fast_with_status(row, row_range, Status::Damaged);
        }
    }
}

fn calc_combinations(input: &str) -> u128 {
    let (_, (row, row_range)) = parse_row_report(input).unwrap();
    let row_range = enrich_row_range(row_range);

    let combinations = calc_combinations_rec(&vec![], &row, &row_range);
    debug::print_report(&vec![], &row, &row_range);
    println!(" => {}", combinations);

    combinations
}

fn calc_combination_sum(input: &str) -> u128 {
    input.lines().map(|line| calc_combinations(line)).sum()
}

fn unfold_report(report: &(Vec<Status>, Vec<StatusRange>)) -> (Vec<Status>, Vec<StatusRange>) {
    let nr_of_copies = 5;
    let mut unfolded_report = (Vec::new(), Vec::new());

    for i in 0..nr_of_copies {
        unfolded_report.0.append(&mut report.0.clone());
        unfolded_report.1.append(&mut report.1.clone());
        if i != nr_of_copies - 1 {
            unfolded_report.0.push(Status::Unknown);
        }
    }

    unfolded_report
}

fn calc_combinations_folded(input: &str) -> u128 {
    let report = parse_row_report(input).unwrap().1;
    let (row, row_range) = unfold_report(&report);
    let row_range = enrich_row_range(row_range);

    let combinations = calc_combinations_rec_fast(row.clone(), row_range.clone());
    debug::print_report(&vec![], &row, &row_range);
    println!(" => {}", combinations);

    combinations
}

fn calc_combination_sum_folded(input: &str) -> u128 {
    input
        .lines()
        .map(|line| calc_combinations_folded(line))
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ten_combinations() {
        let input = "?###???????? 3,2,1";
        assert_eq!(calc_combinations(input), 10)
    }

    #[test]
    fn small_input() {
        // The easiest way to open the data is to include it into the generated binary.
        let input = include_str!("../input/small.txt");
        assert_eq!(calc_combination_sum(input), 21)
    }

    #[test]
    fn large_input() {
        // You can also read the file completely into memory
        let file = std::fs::read_to_string("input/big.txt").expect("Could not open input file");
        assert_eq!(calc_combination_sum(&file), 7286)
    }

    #[test]
    fn test_mini_folded() {
        let input = "???.### 1,1,3";
        assert_eq!(calc_combinations_folded(input), 1)
    }

    #[test]
    fn test_small_folded() {
        // The easiest way to open the data is to include it into the generated binary.
        let input = include_str!("../input/small.txt");
        assert_eq!(calc_combination_sum_folded(input), 525152)
    }

    #[test]
    fn large_input_folded() {
        // You can also read the file completely into memory
        let file = std::fs::read_to_string("input/big.txt").expect("Could not open input file");
        assert_eq!(calc_combination_sum_folded(&file), 25470469710341)
    }
}
