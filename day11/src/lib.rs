use itertools::iproduct;
use nom::InputTake; // cartesian product

mod debug;

#[derive(Debug, PartialEq, Clone)]
pub enum Space {
    Empty,
    Galaxy,
}

fn parse_universe(input: &str) -> Vec<Vec<Space>> {
    input
        .lines()
        .map(|line| {
            line.chars()
                .map(|c| match c {
                    '.' => Space::Empty,
                    '#' => Space::Galaxy,
                    _ => panic!("Unknown space type"),
                })
                .collect()
        })
        .collect()
}

fn transpose(universe: Vec<Vec<Space>>) -> Vec<Vec<Space>> {
    let mut result = Vec::new();
    for x in 0..universe[0].len() {
        let mut row = Vec::new();
        for y in 0..universe.len() {
            row.push(universe[y][x].clone());
        }
        result.push(row);
    }
    result
}

fn get_empty_rows(universe: Vec<Vec<Space>>) -> Vec<usize> {
    universe
        .iter()
        .enumerate()
        .filter(|(_, row)| row.iter().all(|space| *space == Space::Empty))
        .map(|(i, _)| i)
        .collect()
}

fn expand_space(universe: &Vec<Vec<Space>>) -> Vec<Vec<Space>> {
    let empty_rows = get_empty_rows(universe.clone());
    let empty_cols = get_empty_rows(transpose(universe.clone()));

    println!("empty rows: {:?}", empty_rows);
    println!("empty cols: {:?}", empty_cols);

    // duplicate empty rows and empty cols
    let mut result = Vec::new();
    for (y, row) in universe.iter().enumerate() {
        let mut new_row = Vec::new();
        for (x, space) in row.iter().enumerate() {
            if empty_cols.contains(&x) {
                new_row.push(Space::Empty);
            }

            new_row.push(space.clone());
        }
        result.push(new_row);

        if empty_rows.contains(&y) {
            let mut empty_row = Vec::new();
            for _ in 0..result[0].len() {
                empty_row.push(Space::Empty);
            }
            result.push(empty_row);
        }
    }

    result
}

fn expand_space_on_coordinates(
    universe: &Vec<Vec<Space>>,
    coords: Vec<(u128, u128)>,
    expansion_factor: u128,
) -> Vec<(u128, u128)> {
    let empty_rows = get_empty_rows(universe.clone());
    let empty_cols = get_empty_rows(transpose(universe.clone()));

    coords
        .iter()
        .map(|(x, y)| {
            let mut new_x = *x as u128;
            let mut new_y = *y as u128;

            for dx in 0..(*x as usize) {
                if empty_cols.contains(&dx) {
                    new_x += expansion_factor - 1;
                }
            }

            for dy in 0..(*y as usize) {
                if empty_rows.contains(&dy) {
                    new_y += expansion_factor - 1;
                }
            }

            (new_x, new_y)
        })
        .collect()
}

fn get_galaxy_coords(universe: &Vec<Vec<Space>>) -> Vec<(u128, u128)> {
    let mut result = Vec::new();
    for (y, row) in universe.iter().enumerate() {
        for (x, c) in row.iter().enumerate() {
            if let Space::Galaxy = c {
                result.push((x as u128, y as u128));
            }
        }
    }
    result
}

fn calc_distance(a: &(u128, u128), b: &(u128, u128)) -> u128 {
    let (x1, y1) = a;
    let (x2, y2) = b;

    let dist_x = if x1 > x2 { x1 - x2 } else { x2 - x1 };
    let dist_y = if y1 > y2 { y1 - y2 } else { y2 - y1 };

    dist_x + dist_y
}

fn calc_galaxy_distance_sum(input: &str) -> u128 {
    let universe = parse_universe(input);
    debug::print_universe(&universe);

    let universe = expand_space(&universe);
    println!("########################");
    debug::print_universe(&universe);

    let galaxies = get_galaxy_coords(&universe);
    let distances = iproduct!(galaxies.iter(), galaxies.iter())
        .map(|(a, b)| (a, b, calc_distance(a, b)))
        .collect::<Vec<_>>();
    distances.iter().map(|(_, _, d)| d).sum::<u128>() / 2
}

fn calc_galaxy_distance_sum_efficient(input: &str, expansion_factor: u128) -> u128 {
    let universe = parse_universe(input);
    debug::print_universe(&universe);

    let galaxies = get_galaxy_coords(&universe);
    let expanded_galaxies = expand_space_on_coordinates(&universe, galaxies, expansion_factor);
    let distances = iproduct!(expanded_galaxies.iter(), expanded_galaxies.iter())
        .map(|(a, b)| (a, b, calc_distance(a, b)))
        .collect::<Vec<_>>();

    distances.iter().map(|(_, _, d)| d).sum::<u128>() / 2
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_calc_distance() {
        let a = (0, 0);
        let b = (1, 0);
        let expected = 1;
        let result = calc_distance(&a, &b);
        assert_eq!(result, expected);
    }

    #[test]
    fn small_input() {
        // The easiest way to open the data is to include it into the generated binary.
        let input = include_str!("../input/small.txt");
        assert_eq!(calc_galaxy_distance_sum(input), 374)
    }

    #[test]
    fn large_input() {
        // You can also read the file completely into memory
        let file = std::fs::read_to_string("input/big.txt").expect("Could not open input file");
        assert_eq!(calc_galaxy_distance_sum(&file), 9509330)
    }

    #[test]
    fn small_input_efficient_small_expansion() {
        // The easiest way to open the data is to include it into the generated binary.
        let input = include_str!("../input/small.txt");
        assert_eq!(calc_galaxy_distance_sum_efficient(input, 2), 374)
    }

    #[test]
    fn large_input_efficient_small_expansion() {
        // You can also read the file completely into memory
        let file = std::fs::read_to_string("input/big.txt").expect("Could not open input file");
        assert_eq!(calc_galaxy_distance_sum_efficient(&file, 2), 9509330)
    }

    #[test]
    fn small_input_efficient_mid_expansion() {
        // The easiest way to open the data is to include it into the generated binary.
        let input = include_str!("../input/small.txt");
        assert_eq!(calc_galaxy_distance_sum_efficient(input, 10), 1030)
    }

    #[test]
    fn small_input_efficient_bigger_expansion() {
        // The easiest way to open the data is to include it into the generated binary.
        let input = include_str!("../input/small.txt");
        assert_eq!(calc_galaxy_distance_sum_efficient(input, 100), 8410)
    }

    #[test]
    fn large_input_efficient_huge_expansion() {
        // You can also read the file completely into memory
        let file = std::fs::read_to_string("input/big.txt").expect("Could not open input file");
        assert_eq!(calc_galaxy_distance_sum_efficient(&file, 1000000), 635832237682)
    }
}
