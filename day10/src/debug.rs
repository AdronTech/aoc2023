use colored::*;

use crate::{Pipe, ExpandedPipe};

fn map_char(p: &Pipe) -> char {
    match p {
        Pipe::Start => '┼',
        Pipe::Vertical => '│',
        Pipe::Horizontal => '─',
        Pipe::NorthEast => '└',
        Pipe::NorthWest => '┘',
        Pipe::SouthWest => '┐',
        Pipe::SouthEast => '┌',
        Pipe::Empty => '.',
    }
}

pub fn print_pipes(pipes: &Vec<Vec<Pipe>>) {
    for line in pipes {
        for pipe in line {
            let c = map_char(pipe);
            match pipe {
                Pipe::Start => print!("{}", c.to_string().green()),
                Pipe::Empty => print!("{}", ".".black()),
                _ => print!("{}", c),
            }
        }
        println!();
    }
}

pub fn print_pipes_connected_to_start(pipes: &Vec<Vec<Pipe>>, distances: &Vec<Vec<Option<u32>>>) {
    for (y, line) in pipes.iter().enumerate() {
        for (x, pipe) in line.iter().enumerate() {
            if let Some(distance) = distances[y][x] {
                print!("{}", map_char(pipe).to_string().yellow());
            } else {
                match pipe {
                    Pipe::Start => print!("{}", map_char(pipe).to_string().green()),
                    Pipe::Empty => print!("{}", ".".black()),
                    _ => print!("{}", map_char(pipe)),
                }
            }
        }
        println!();
    }
}

pub fn print_distances(distances: &Vec<Vec<Option<u32>>>) {
    for line in distances {
        for distance in line {
            match distance {
                Some(d) => print!("{:1}", d.to_string().yellow()),
                None => print!("{}", ".".black()),
            }
        }
        println!();
    }
}

pub fn print_expanded_pipes(expanded_pipes: &Vec<Vec<ExpandedPipe>>) {
    for line in expanded_pipes {
        for pipe in line {
            match pipe {
                ExpandedPipe::Pipe(_) => print!("{}", "#".yellow()),
                ExpandedPipe::Unknown => print!("{}", ".".black()),
                ExpandedPipe::Inside => print!("{}", "I".green()),
                ExpandedPipe::Outside => print!("{}", "O".red()),
            }
        }
        println!();
    }
}

pub fn print_reduced_pipes(reduced_pipes: &Vec<Vec<ExpandedPipe>>) {
    for line in reduced_pipes {
        for pipe in line {
            match pipe {
                ExpandedPipe::Pipe(p) => print!("{}", map_char(p).to_string().yellow()),
                ExpandedPipe::Unknown => print!("{}", ".".black()),
                ExpandedPipe::Inside => print!("{}", "I".green()),
                ExpandedPipe::Outside => print!("{}", "O".red()),
            }
        }
        println!();
    }
}