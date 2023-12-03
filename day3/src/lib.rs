use std::f32::consts::E;

mod debug;

#[derive(Debug)]
pub enum EnginePart {
    Gear(u32),
    Unknown(char),
}

#[derive(Debug)]
pub enum SchmaticCell {
    Empty,
    Part(EnginePart),
    PotentialPartNumber(u8),
    PartialPartNumber(u8, (usize, usize)),
}

// parse the input into a 2d array of SchmaticCell
fn parse_schematic(input: &str) -> Vec<Vec<SchmaticCell>> {
    input
        .lines()
        .map(|line| {
            line.chars()
                .map(|c| match c {
                    '.' => SchmaticCell::Empty,
                    x if x.is_digit(10) => {
                        SchmaticCell::PotentialPartNumber(c.to_digit(10).unwrap() as u8)
                    }
                    c => SchmaticCell::Part(EnginePart::Unknown(c)),
                })
                .collect::<Vec<SchmaticCell>>()
        })
        .collect::<Vec<Vec<SchmaticCell>>>()
}

fn flood_transform_potential_part_numbers(
    schematic: &mut Vec<Vec<SchmaticCell>>,
    x: i32,
    y: i32,
    part_coords: &(usize, usize),
) {
    if x < 0 || y < 0 || y >= schematic.len() as i32 || x >= schematic[y as usize].len() as i32 {
        return;
    }

    match schematic[y as usize][x as usize] {
        SchmaticCell::PotentialPartNumber(n) => {
            schematic[y as usize][x as usize] =
                SchmaticCell::PartialPartNumber(n, part_coords.clone());
            flood_transform_potential_part_numbers(schematic, x + 1, y, part_coords);
            flood_transform_potential_part_numbers(schematic, x - 1, y, part_coords);
        }
        _ => (),
    }
}

// transform potential part numbers into part numbers if they are adjacent to a part
fn transform_potential_part_numbers(schematic: &mut Vec<Vec<SchmaticCell>>) {
    for y in 0..schematic.len() {
        for x in 0..schematic[y].len() {
            if let SchmaticCell::Part(_) = schematic[y][x] {
                for dx in (-1 as i32)..=1 {
                    for dy in (-1 as i32)..=1 {
                        if dx == 0 && dy == 0 {
                            continue;
                        }

                        flood_transform_potential_part_numbers(
                            schematic,
                            (x as i32) + dx,
                            (y as i32) + dy,
                            &(x, y),
                        );
                    }
                }
            }
        }
    }
}

fn extract_part_numbers(schematic: &Vec<Vec<SchmaticCell>>) -> Vec<u32> {
    let mut part_numbers = Vec::new();

    // combine adjecent partial part numbers into a single part number
    let mut part_number = String::new();
    for y in 0..schematic.len() {
        for x in 0..schematic[y].len() {
            match schematic[y][x] {
                SchmaticCell::PartialPartNumber(n, _) => part_number.push_str(&n.to_string()),
                _ => {
                    if !part_number.is_empty() {
                        part_numbers.push(part_number.parse::<u32>().unwrap());
                        part_number.clear();
                    }
                }
            }
        }
        if !part_number.is_empty() {
            part_numbers.push(part_number.parse::<u32>().unwrap());
            part_number.clear();
        }
    }

    part_numbers
}

fn calc_partnumber_sum(input: &str) -> u32 {
    let mut schematic = parse_schematic(input);
    debug::print_schematic(&schematic);
    println!("-------------------");
    transform_potential_part_numbers(&mut schematic);
    debug::print_schematic(&schematic);

    let part_numbers = extract_part_numbers(&schematic);
    println!("{:?}", part_numbers);

    part_numbers.iter().sum()
}

fn get(x: i32, y: i32, schematic: &Vec<Vec<SchmaticCell>>) -> Option<&SchmaticCell> {
    if x < 0 || y < 0 || y >= schematic.len() as i32 || x >= schematic[y as usize].len() as i32 {
        return None;
    }

    Some(&schematic[y as usize][x as usize])
}

fn get_part_numbers_flood(
    schematic: &Vec<Vec<SchmaticCell>>,
    x: i32,
    y: i32,
    already_checked: &mut Vec<(i32, i32)>,
) -> Option<u32> {
    if already_checked.contains(&(x, y)) {
        return;
    }
    already_checked.push((x, y));

    let mut part_numbers = Vec::new();

    if let Some(SchmaticCell::PartialPartNumber(n, _)) = get(x, y, schematic) {
        flood_get_part_numbers_around(schematic, x - 1, y, already_checked);
        flood_get_part_numbers_around(schematic, x + 1, y, already_checked);
    }


}

fn get_part_numbers_around(schematic: &Vec<Vec<SchmaticCell>>, x: i32, y: i32) -> Vec<u32> {
    let mut part_numbers = Vec::new();
    let mut already_checked = Vec::new();

    for dx in (-1 as i32)..=1 {
        for dy in (-1 as i32)..=1 {
            if dx == 0 && dy == 0 {
                continue;
            }

            if let Some(part_number) = get_part_numbers_flood(
                schematic,
                (x as i32) + dx,
                (y as i32) + dy,
                &mut already_checked,
            ) {
                part_numbers.push(part_number);
            }
        }
    }

    part_numbers
}

fn mark_gears(schematic: &mut Vec<Vec<SchmaticCell>>) {
    for y in 0..schematic.len() {
        for x in 0..schematic[y].len() {
            if let SchmaticCell::Part(EnginePart::Unknown('*')) = schematic[y][x] {
                // let part_numbers = get_part_numbers_around(schematic, x as i32, y as i32);
                // if part_numbers.len() != 2 {
                //     continue;
                // }
                // let gear_ratio = part_numbers[0] * part_numbers[1];
                let gear_ratio = 0;

                schematic[y][x] = SchmaticCell::Part(EnginePart::Gear(gear_ratio));
            }
        }
    }
}

fn calc_gear_ratio_sum(input: &str) -> u32 {
    let mut schematic = parse_schematic(input);
    debug::print_schematic(&schematic);
    println!("-------------------");
    transform_potential_part_numbers(&mut schematic);
    debug::print_schematic(&schematic);
    println!("-------------------");
    mark_gears(&mut schematic);
    debug::print_schematic(&schematic);

    let mut gear_ratio_sum = 0;
    for y in 0..schematic.len() {
        for x in 0..schematic[y].len() {
            if let SchmaticCell::Part(EnginePart::Gear(gear_ratio)) = schematic[y][x] {
                gear_ratio_sum += gear_ratio;
            }
        }
    }

    gear_ratio_sum
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn small_input() {
        // The easiest way to open the data is to include it into the generated binary.
        let input = include_str!("../input/small.txt");
        assert_eq!(calc_partnumber_sum(input), 4361)
    }

    #[test]
    fn large_input() {
        // You can also read the file completely into memory
        let file = std::fs::read_to_string("input/big.txt").expect("Could not open input file");
        assert_eq!(calc_partnumber_sum(&file), 0)
    }

    #[test]
    fn small_input_power_sum() {
        // The easiest way to open the data is to include it into the generated binary.
        let input = include_str!("../input/small.txt");
        assert_eq!(calc_gear_ratio_sum(input), 0)
    }

    // #[test]
    // fn large_input_power_sum() {
    //     // You can also read the file completely into memory
    //     let file = std::fs::read_to_string("input/big.txt").expect("Could not open input file");
    //     assert_eq!(calculate_power_sum(&file), 71535)
    // }
}
