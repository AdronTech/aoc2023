use colored::*;

use crate::Pipe;

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