use nom::IResult;

mod debug;

struct Range {
    src_start: u32,
    dest_start: u32,
    length: u32,
}

struct Map {
    ranges: Vec<Range>,
}

struct Almanac {
    seeds: Vec<u32>,
    maps: Vec<Map>,
}

fn parse_range(input: &str) -> IResult<&str, Range> {
    todo!()
}

fn parse_almanac(input: &str) -> Almanac {
    todo!()
}

fn get_min_location_number(input: &str) -> u32 {

    let almanac = parse_almanac(input);
    todo!()
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn small_input() {
        // The easiest way to open the data is to include it into the generated binary.
        let input = include_str!("../input/small.txt");
        assert_eq!(get_min_location_number(input), 35)
    }

    // #[test]
    // fn large_input() {
    //     // You can also read the file completely into memory
    //     let file = std::fs::read_to_string("input/big.txt").expect("Could not open input file");
    //     assert_eq!(calc_score_of_all_cards(&file), 26426)
    // }

    // #[test]
    // fn small_input_power_sum() {
    //     // The easiest way to open the data is to include it into the generated binary.
    //     let input = include_str!("../input/small.txt");
    //     assert_eq!(calc_accumulated_score_of_all_cards(input), 30)
    // }

    // #[test]
    // fn large_input_power_sum() {
    //     // You can also read the file completely into memory
    //     let file = std::fs::read_to_string("input/big.txt").expect("Could not open input file");
    //     assert_eq!(calc_accumulated_score_of_all_cards(&file), 6227972)
    // }
}
