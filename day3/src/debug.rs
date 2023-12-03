use colored::*;
use crate::{SchematicCell, EnginePart};

// print schematic
pub fn print_schematic(schematic: &Vec<Vec<SchematicCell>>) {
    schematic.iter().for_each(|row| {
        row.iter().for_each(|cell| {
            match cell {
                SchematicCell::Empty => print!("{}", ".".black()),
                SchematicCell::Part(EnginePart::Gear(_)) => print!("{}", "*".yellow()),
                SchematicCell::Part(EnginePart::Unknown(c)) => print!("{}", c.to_string().blue()),
                SchematicCell::PartialPartNumber(n, part_coords) => {
                    if let SchematicCell::Part(EnginePart::Gear(_)) = schematic[part_coords.1][part_coords.0] {
                        print!("{}", n.to_string().yellow())
                    } else {
                        print!("{}", n.to_string().green())
                    }
                },
                SchematicCell::PotentialPartNumber(n) => print!("{}", n.to_string().red()),
            }
        });
        println!();
    });
}
