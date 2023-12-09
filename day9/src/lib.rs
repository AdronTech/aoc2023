use nom::{
    character::complete::{space1, i32 as i32_parser},
    multi::separated_list1,
    IResult,
};

mod debug;

fn parse_history(input: &str) -> IResult<&str, Vec<i32>> {
    separated_list1(space1, i32_parser)(input)
}

fn predict_next(history: Vec<i32>) -> i32 {
    let differences = history
        .iter()
        .zip(history.iter().skip(1))
        .map(|(a, b)| b - a)
        .collect::<Vec<_>>();

    if differences.iter().all(|d| *d == 0) {
        return history[history.len() - 1]
    }

    let next_difference = predict_next(differences);
    history[history.len() - 1] + next_difference
}

fn predict_prev(history: Vec<i32>) -> i32 {
    let differences = history
        .iter()
        .zip(history.iter().skip(1))
        .map(|(a, b)| b - a)
        .collect::<Vec<_>>();

    if differences.iter().all(|d| *d == 0) {
        return history[0]
    }

    let prev_difference = predict_prev(differences);
    history[0] - prev_difference
}

fn calc_history_next_prediction_sum(input: &str) -> i32 {
    input
        .lines()
        .map(|line| parse_history(line).unwrap().1)
        .map(|history| predict_next(history))
        .sum()
}

fn calc_history_prev_prediction_sum(input: &str) -> i32 {
    input
        .lines()
        .map(|line| parse_history(line).unwrap().1)
        .map(|history| predict_prev(history))
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn small_input() {
        // The easiest way to open the data is to include it into the generated binary.
        let input = include_str!("../input/small.txt");
        assert_eq!(calc_history_next_prediction_sum(input), 114)
    }

    #[test]
    fn large_input() {
        // You can also read the file completely into memory
        let file = std::fs::read_to_string("input/big.txt").expect("Could not open input file");
        assert_eq!(calc_history_next_prediction_sum(&file), 0)
    }

    #[test]
    fn small_input_second_example() {
        // The easiest way to open the data is to include it into the generated binary.
        let input = include_str!("../input/small.txt");
        assert_eq!(calc_history_prev_prediction_sum(input), 2)
    }

    #[test]
    fn large_input_complicated() {
        // You can also read the file completely into memory
        let file = std::fs::read_to_string("input/big.txt").expect("Could not open input file");
        assert_eq!(calc_history_prev_prediction_sum(&file), 1022)
    }
}
