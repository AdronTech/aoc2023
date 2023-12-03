use crate::{PartType, SchematicCell};
use colored::*;

// print schematic
pub fn print_schematic(schematic: &Vec<Vec<SchematicCell>>) {
    schematic.iter().for_each(|row| {
        row.iter().for_each(|cell| match cell {
            SchematicCell::Empty => print!("{}", ".".black()),
            SchematicCell::Part(p) => match p.as_ref().borrow().part_type {
                PartType::Gear => print!("{}", "*".yellow()),
                PartType::Unknown(c) => print!("{}", c.to_string().blue()),
            },
            SchematicCell::PartialPartNumber(n, part_number) => {
                match &part_number.borrow().part {
                    Some(p) => match p.as_ref().borrow().part_type {
                        PartType::Gear => print!("{}", n.to_string().yellow()),
                        PartType::Unknown(_) => print!("{}", n.to_string().blue()),
                    },
                    None => print!("{}", n.to_string().red()),
                }
            }
            SchematicCell::PotentialPartNumber(n) => print!("{}", n.to_string().black()),
        });
        println!();
    });
}
