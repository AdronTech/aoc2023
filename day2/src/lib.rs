use colored::*;
mod debug;

#[derive(Debug)]
struct CubeSet {
    red: u32,
    green: u32,
    blue: u32,
}

fn parse_game_set(input: &str) -> CubeSet {
    // input = 3 blue, 4 red
    let mut cubes = CubeSet {
        red: 0,
        green: 0,
        blue: 0,
    };

    input
        .split(',')
        .map(|cube| cube.trim())
        .for_each(|cube| match cube.split_once(' ') {
            Some((count, color)) => match color {
                "red" => cubes.red += count.parse::<u32>().unwrap(),
                "green" => cubes.green += count.parse::<u32>().unwrap(),
                "blue" => cubes.blue += count.parse::<u32>().unwrap(),
                _ => panic!("Unknown color: {}", color),
            },
            None => panic!("Invalid cube: {}", cube),
        });

    cubes
}

fn is_cube_set_possible(cube_set: &CubeSet, initial_cubes: &CubeSet) -> bool {
    cube_set.red <= initial_cubes.red
        && cube_set.green <= initial_cubes.green
        && cube_set.blue <= initial_cubes.blue
}

fn parse_game(input: &str) -> Vec<CubeSet> {
    // input = Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
    input
        .split_once(':')
        .map(|(_game, game_sets)| game_sets)
        .unwrap()
        .split_terminator(';')
        .map(|cube| cube.trim())
        .map(|cube| parse_game_set(cube))
        .collect::<Vec<CubeSet>>()
}

fn check_game(input: &str, initial_cubes: &CubeSet) -> bool {
    let game = parse_game(input);
    game.iter()
        .all(|cube_set| is_cube_set_possible(cube_set, initial_cubes))
}

fn check_games(input: &str, initial_cubes: &CubeSet) -> usize {
    input
        .lines()
        .map(|line| check_game(line, initial_cubes))
        .enumerate()
        // .inspect(|(i, valid)| {
        //     // green if valid, red if invalid
        //     if *valid {
        //         print!("{}", i.to_string().green());
        //     } else {
        //         print!("{}", i.to_string().red());
        //     }
        // })
        .filter(|(_i, valid)| *valid)
        .map(|(i, _valid)| i + 1)
        .sum()
}

fn calculate_cube_set_power(cube_set: &CubeSet) -> u32 {
    cube_set.red * cube_set.green * cube_set.blue
}

fn get_minimum_cube_set(cube_sets: &Vec<CubeSet>) -> CubeSet {
    let mut min_cube_set = CubeSet {
        red: 0,
        green: 0,
        blue: 0,
    };

    cube_sets.iter().for_each(|cube_set| {
        if cube_set.red > min_cube_set.red {
            min_cube_set.red = cube_set.red;
        }
        if cube_set.green > min_cube_set.green {
            min_cube_set.green = cube_set.green;
        }
        if cube_set.blue > min_cube_set.blue {
            min_cube_set.blue = cube_set.blue;
        }
    });

    min_cube_set
}

fn calculate_game_power(input: &str) -> u32 {
    let game = parse_game(input);
    let min_cube_set = get_minimum_cube_set(&game);
    // println!("{:?}", min_cube_set);
    calculate_cube_set_power(&min_cube_set)
}

fn calculate_power_sum(input: &str) -> u32 {
    input
        .lines()
        .map(calculate_game_power)
        // .inspect(|power| println!("{}", power))
        .sum()
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn small_input() {
        // The easiest way to open the data is to include it into the generated binary.
        let input = include_str!("../input/small.txt");
        assert_eq!(
            check_games(
                input,
                &CubeSet {
                    red: 12,
                    green: 13,
                    blue: 14,
                }
            ),
            8
        )
    }

    #[test]
    fn large_input() {
        // You can also read the file completely into memory
        let file = std::fs::read_to_string("input/big.txt").expect("Could not open input file");
        assert_eq!(
            check_games(
                &file,
                &CubeSet {
                    red: 12,
                    green: 13,
                    blue: 14,
                }
            ),
            2720
        )
    }

    #[test]
    fn small_input_power_sum() {
        // The easiest way to open the data is to include it into the generated binary.
        let input = include_str!("../input/small.txt");
        assert_eq!(calculate_power_sum(input), 2286)
    }

    #[test]
    fn large_input_power_sum() {
        // You can also read the file completely into memory
        let file = std::fs::read_to_string("input/big.txt").expect("Could not open input file");
        assert_eq!(calculate_power_sum(&file), 71535)
    }
}
