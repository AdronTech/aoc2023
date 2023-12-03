use lazy_static::lazy_static;
use regex::Regex;
use colored::*;

mod debug;

lazy_static! {
    static ref RE_DIGIT: regex::Regex = Regex::new(r"1|2|3|4|5|6|7|8|9").unwrap();
    static ref RE_EXT: regex::Regex = Regex::new(r"one|two|three|four|five|six|seven|eight|nine|1|2|3|4|5|6|7|8|9").unwrap();
    static ref RE_EXT_REV: regex::Regex = Regex::new(r"enin|thgie|neves|xis|evif|ruof|eerht|owt|eno|9|8|7|6|5|4|3|2|1").unwrap();
}

fn find_first_occurance_of_digit(input: &str, re: &regex::Regex) -> Option<(usize, usize)> {
    re.find(input).map(|m| (m.start(), m.end()))
}

fn find_last_occurance_of_digit(input: &str, re_rev: &regex::Regex) -> Option<(usize, usize)> {
    let rev_line: String = input.chars().rev().collect();
    re_rev.find(&rev_line).map(|m| (m.start(), m.end()))    
    .map(|(start, end)| (input.len() - end, input.len() - start))
}

fn parse_digit(input: &str) -> u32 {
    match input {
        "one" => 1,
        "two" => 2,
        "three" => 3,
        "four" => 4,
        "five" => 5,
        "six" => 6,
        "seven" => 7,
        "eight" => 8,
        "nine" => 9,
        _ => input.parse().unwrap(),
    }
}

fn calibrate_line(line: &str, re: &regex::Regex, re_rev: &regex::Regex) -> u32 {
    let first_occurrence = find_first_occurance_of_digit(line, re).unwrap();
    let last_occurrence = find_last_occurance_of_digit(line, re_rev).unwrap();

    let colored_line = debug::debug_color_line(line, first_occurrence, last_occurrence);
    print!("{:>120} ({:3})", colored_line, colored_line.chars().count());

    let (f_start, f_end) = first_occurrence;
    let (l_start, l_end) = last_occurrence;
    let f_digit = parse_digit(&line[f_start..f_end]);
    let l_digit = parse_digit(&line[l_start..l_end]);
    let number = format!("{}{}", f_digit, l_digit).parse::<u32>().unwrap();

    print!(" -> ");
    println!("{}", number.to_string().as_str().red());

    number
}

fn generate_calibration(input: &str) -> u32 {
    input.lines().map(|line| calibrate_line(line, &RE_DIGIT, &RE_DIGIT)).sum()
}

fn generate_calibration_extended(input: &str) -> u32 {
    input.lines().map(|line| calibrate_line(line, &RE_EXT, &RE_EXT_REV)).sum()
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn small_input() {
        // The easiest way to open the data is to include it into the generated binary.
        let input = include_str!("../input/small.txt");
        assert_eq!(generate_calibration(input), 142)
    }

    #[test]
    fn large_input() {
        // You can also read the file completely into memory
        let file = std::fs::read_to_string("input/big.txt").expect("Could not open input file");
        assert_eq!(generate_calibration(&file), 54708)
    }

    #[test]
    fn small_input_extended() {
        // The easiest way to open the data is to include it into the generated binary.
        let input = include_str!("../input/small_extended.txt");
        assert_eq!(generate_calibration_extended(input), 281)
    }

    #[test]
    fn large_input_extended() {
        // You can also read the file completely into memory
        let file = std::fs::read_to_string("input/big.txt").expect("Could not open input file");
        assert_eq!(generate_calibration_extended(&file), 54087)
    }
}
