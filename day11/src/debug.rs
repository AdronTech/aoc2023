use colored::*;

use crate::Space;

pub fn print_universe(universe: &Vec<Vec<Space>>) {
    for line in universe {
        for space in line {
            match space {
                Space::Empty => print!("{}", ".".black()),
                Space::Galaxy => print!("{}", "#".cyan()),
            }
        }
        println!();
    }
}