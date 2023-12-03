use colored::*;
use crate::{SchmaticCell, EnginePart};

// print schematic
pub fn print_schematic(schematic: &Vec<Vec<SchmaticCell>>) {
    schematic.iter().for_each(|row| {
        row.iter().for_each(|cell| {
            match cell {
                SchmaticCell::Empty => print!("{}", ".".black()),
                SchmaticCell::Part(EnginePart::Gear(_)) => print!("{}", "*".yellow()),
                SchmaticCell::Part(EnginePart::Unknown(c)) => print!("{}", c.to_string().blue()),
                SchmaticCell::PartialPartNumber(n, part_coords) => {
                    if let SchmaticCell::Part(EnginePart::Gear(_)) = schematic[part_coords.1][part_coords.0] {
                        print!("{}", n.to_string().yellow())
                    } else {
                        print!("{}", n.to_string().green())
                    }
                },
                SchmaticCell::PotentialPartNumber(n) => print!("{}", n.to_string().red()),
            }
        });
        println!();
    });
}
