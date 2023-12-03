use std::{rc::Rc, cell::RefCell};

mod debug;

#[derive(Debug)]
pub enum PartType {
    Gear,
    Unknown(char),
}

#[derive(Debug)]
pub struct Part {
    pub part_type: PartType,
    pub part_numbers: Vec<Rc<RefCell<PartNumber>>>,
}

impl Part {
    pub fn new(part_type: PartType) -> Self {
        Self {
            part_type,
            part_numbers: Vec::new(),
        }
    }
}

#[derive(Debug)]
pub struct PartNumber {
    pub value: u32,
    pub part: Option<Rc<RefCell<Part>>>,
}

impl PartNumber {
    pub fn new(value: u32) -> Self {
        Self { value, part: None }
    }
}

#[derive(Debug)]
pub enum SchematicCell {
    Empty,
    Part(Rc<RefCell<Part>>),
    PotentialPartNumber(char),
    PartialPartNumber(char, Rc<RefCell<PartNumber>>),
}

// alias schematic as a 2d array of SchematicCell
type Schematic = Vec<Vec<SchematicCell>>;

// parse the input into a 2d array of SchmaticCell
fn parse_schematic(input: &str) -> Schematic {
    input
        .lines()
        .map(|line| {
            line.chars()
                .map(|c| match c {
                    '.' => SchematicCell::Empty,
                    d if d.is_digit(10) => SchematicCell::PotentialPartNumber(d),
                    c => SchematicCell::Part(Rc::new(RefCell::new(Part::new(PartType::Unknown(c))))),
                })
                .collect::<Vec<SchematicCell>>()
        })
        .collect::<Schematic>()
}

fn transform_part_numbers(schematic: &mut Schematic) {
    let mut current_part_number: Option<Rc<RefCell<PartNumber>>> = None;
    for y in 0..schematic.len() {
        for x in 0..schematic[y].len() {
            if let SchematicCell::PotentialPartNumber(digit) = schematic[y][x] {
                if let None = &current_part_number {
                    current_part_number = Some(Rc::new(RefCell::new(PartNumber::new(digit.to_digit(10).unwrap()))));
                } else if let Some(current_part_number) = &current_part_number {
                    let mut current_part_number = current_part_number.as_ref().borrow_mut();
                    current_part_number.value *= 10;
                    current_part_number.value += digit.to_digit(10).unwrap();
                }

                schematic[y][x] = SchematicCell::PartialPartNumber(
                    digit,
                    Rc::clone(current_part_number.as_ref().unwrap()),
                );
            } else {
                current_part_number = None;
            }
        }
    }
}

fn assign_part_number(part: &Rc<RefCell<Part>>, part_number: &Rc<RefCell<PartNumber>>) {
    if part
    .borrow()
    .part_numbers
    .iter()
    .any(|item| Rc::ptr_eq(item, part_number))
    {
        return;
    }
    
    part.as_ref().borrow_mut().part_numbers.push(Rc::clone(part_number));
    part_number.as_ref().borrow_mut().part = Some(Rc::clone(part));
}

fn assign_part_numbers(schematic: &mut Schematic) {
    for y in 0..schematic.len() {
        for x in 0..schematic[y].len() {
            if let SchematicCell::Part(part) = &schematic[y][x] {
                // assign part to part numbers in all neighbiouring cells
                for dy in (-1 as i32)..=1 {
                    for dx in (-1 as i32)..=1 {
                        if dy == 0 && dx == 0 {
                            continue;
                        }
                        let nx = x as i32 + dx;
                        let ny = y as i32 + dy;
                        if nx < 0 || nx >= schematic[y].len() as i32 {
                            continue;
                        }
                        if ny < 0 || ny >= schematic.len() as i32 {
                            continue;
                        }

                        if let SchematicCell::PartialPartNumber(_, part_number) =
                            &schematic[ny as usize][nx as usize]
                        {
                            assign_part_number(part, part_number);
                        }
                    }
                }
            }
        }
    }
}

fn transform_to_geared_parts(schematic: &mut Schematic) {
    for y in 0..schematic.len() {
        for x in 0..schematic[y].len() {
            if let SchematicCell::Part(part) = &schematic[y][x] {
                let mut p = part.as_ref().borrow_mut();

                if let PartType::Unknown('*') = p.part_type {
                    if p.part_numbers.len() != 2 {
                        continue;
                    }

                    p.part_type = PartType::Gear;
                }
            }
        }
    }
}

fn get_all_valid_part_numbers(schematic: &Schematic) -> Vec<Rc<RefCell<PartNumber>>> {
    let mut part_numbers = Vec::new();
    for y in 0..schematic.len() {
        for x in 0..schematic[y].len() {
            if let SchematicCell::Part(part) = &schematic[y][x] {
                let p = part.as_ref().borrow();
                part_numbers.extend(p.part_numbers.iter().cloned());
            }
        }
    }
    part_numbers
    
}

fn calc_partnumber_sum(input: &str) -> u32 {
    let mut schematic = parse_schematic(input);
    transform_part_numbers(&mut schematic);
    assign_part_numbers(&mut schematic);
    debug::print_schematic(&schematic);
    println!("----------------------------------------");

    transform_to_geared_parts(&mut schematic);
    debug::print_schematic(&schematic);

    get_all_valid_part_numbers(&schematic).iter().map(|part_number| {
        part_number.as_ref().borrow().value
    }).sum()
}

fn get_all_gear_parts(schematic: &Schematic) -> Vec<Rc<RefCell<Part>>> {
    let mut gear_parts = Vec::new();
    for y in 0..schematic.len() {
        for x in 0..schematic[y].len() {
            if let SchematicCell::Part(part) = &schematic[y][x] {
                let p = part.as_ref().borrow();
                if let PartType::Gear = p.part_type {
                    gear_parts.push(Rc::clone(part));
                }
            }
        }
    }

    gear_parts
}

fn calc_gear_ratio_sum(input: &str) -> u32 {
    let mut schematic = parse_schematic(input);
    transform_part_numbers(&mut schematic);
    assign_part_numbers(&mut schematic);
    debug::print_schematic(&schematic);
    println!("----------------------------------------");

    transform_to_geared_parts(&mut schematic);
    debug::print_schematic(&schematic);

    get_all_gear_parts(&schematic).iter().map(|gear_part| {
        // multiply all part numbers
        gear_part.as_ref().borrow().part_numbers.iter().map(|part_number| {
            part_number.as_ref().borrow().value
        }).product::<u32>()
    }).sum()
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
        assert_eq!(calc_partnumber_sum(&file), 553825)
    }

    #[test]
    fn small_input_power_sum() {
        // The easiest way to open the data is to include it into the generated binary.
        let input = include_str!("../input/small.txt");
        assert_eq!(calc_gear_ratio_sum(input), 467835)
    }

    #[test]
    fn large_input_power_sum() {
        // You can also read the file completely into memory
        let file = std::fs::read_to_string("input/big.txt").expect("Could not open input file");
        assert_eq!(calc_gear_ratio_sum(&file), 93994191)
    }
}
