use nom::{
    bytes::complete::tag,
    character::complete::{line_ending, space1, u64 as u64_parser},
    combinator::map,
    multi::separated_list1,
    sequence::{preceded, separated_pair, terminated},
};
mod debug;

fn parse_races(input: &str) -> Vec<(u64, u64)> {
    let time_label =
        terminated::<&str, _, _, (&str, nom::error::ErrorKind), _, _>(tag("Time:"), space1);
    let distance_label = terminated(tag("Distance:"), space1);

    let time_parser = preceded(time_label, separated_list1(space1, u64_parser));
    let distance_parser = preceded(distance_label, separated_list1(space1, u64_parser));

    map(
        separated_pair(time_parser, line_ending, distance_parser),
        |(time, distance)| {
            time.iter()
                .zip(distance.iter())
                .map(|(t, d)| (*t, *d))
                .collect()
        },
    )(input)
    .unwrap()
    .1
}

fn solve_quadratic_formula(a: f64, b: f64, c: f64) -> (f64, f64) {
    let discriminant = b.powf(2.0) - 4.0 * a * c;
    let root = (discriminant as f64).sqrt();
    let denominator = 2.0 * a;
    let x1 = (-b + root) / denominator;
    let x2 = (-b - root) / denominator;
    (x1, x2)
}

fn calculate_bounds(duration: u64, record: u64) -> (u64, u64) {
    // t * (d - t) = r => t^2 - d * t + r = 0
    // a => 1; b => -d; c => r
    let duration = duration as f64;
    let record = record as f64;
    let (x1, x2) = solve_quadratic_formula(1.0, -duration, record);
    println!("ff {x1} {x2}");
    (x1.ceil() as u64 - 1, x2.floor() as u64 + 1)
}

fn calc_record_product(input: &str) -> u64 {
    let races = parse_races(input);

    races
        .iter()
        .inspect(|(duration, record)| print!("{} {} => ", duration, record))
        .map(|(duration, record)| calculate_bounds(*duration, *record))
        .inspect(|(x1, x2)| println!("{} {}", x1, x2))
        .map(|(x1, x2)| x1 - x2 + 1)
        .product()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_races() {
        let input = "Time:      7  15   30
Distance:  9  40  200";
        let expected = vec![(7, 9), (15, 40), (30, 200)];
        let result = parse_races(input);
        assert_eq!(result, expected);
    }

    #[test]
    fn test_solve_quadratic_formula() {
        let a = 1.0;
        let b = 2.0;
        let c = 1.0;
        let expected = (-1.0, -1.0);
        let result = solve_quadratic_formula(a, b, c);
        assert_eq!(result, expected);
    }

    #[test]
    fn small_input() {
        env_logger::init();

        // The easiest way to open the data is to include it into the generated binary.
        let input = include_str!("../input/small.txt");
        assert_eq!(calc_record_product(input), 288)
    }

    #[test]
    fn large_input() {
        // You can also read the file completely into memory
        let file = std::fs::read_to_string("input/big.txt").expect("Could not open input file");
        assert_eq!(calc_record_product(&file), 1624896)
    }

    #[test]
    fn small_input_kerning() {
        // The easiest way to open the data is to include it into the generated binary.
        let input = include_str!("../input/small_kerning.txt");
        assert_eq!(calc_record_product(input), 71503)
    }

    #[test]
    fn large_input_kerning() {
        // You can also read the file completely into memory
        let file = std::fs::read_to_string("input/big_kerning.txt").expect("Could not open input file");
        assert_eq!(calc_record_product(&file), 32583852)
    }
}
