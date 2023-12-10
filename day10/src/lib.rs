use queues::*;
use std::collections::HashSet;

mod debug;

#[derive(Debug, PartialEq, Eq, Hash)]
enum Port {
    North,
    East,
    South,
    West,
}

impl Port {
    fn opposite(&self) -> Self {
        match self {
            Port::North => Port::South,
            Port::East => Port::West,
            Port::South => Port::North,
            Port::West => Port::East,
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
enum Pipe {
    Start,
    Vertical,
    Horizontal,
    NorthEast,
    NorthWest,
    SouthWest,
    SouthEast,
    Empty,
}

impl Pipe {
    fn new(c: char) -> Self {
        match c {
            '|' => Pipe::Vertical,
            '-' => Pipe::Horizontal,
            'S' => Pipe::Start,
            'L' => Pipe::NorthEast,
            'J' => Pipe::NorthWest,
            '7' => Pipe::SouthWest,
            'F' => Pipe::SouthEast,
            '.' => Pipe::Empty,
            _ => panic!("Unknown pipe: {}", c),
        }
    }

    fn get_ports(&self) -> HashSet<Port> {
        match self {
            Pipe::Start => HashSet::from([Port::North, Port::East, Port::South, Port::West]),
            Pipe::Vertical => HashSet::from([Port::North, Port::South]),
            Pipe::Horizontal => HashSet::from([Port::East, Port::West]),
            Pipe::NorthEast => HashSet::from([Port::North, Port::East]),
            Pipe::NorthWest => HashSet::from([Port::North, Port::West]),
            Pipe::SouthWest => HashSet::from([Port::South, Port::West]),
            Pipe::SouthEast => HashSet::from([Port::South, Port::East]),
            Pipe::Empty => HashSet::from([]),
        }
    }
}

fn parse_pipes(input: &str) -> Vec<Vec<Pipe>> {
    input
        .lines()
        .map(|line| line.chars().map(Pipe::new).collect::<Vec<_>>())
        .collect()
}

fn find_start(pipes: &Vec<Vec<Pipe>>) -> (usize, usize) {
    for (y, line) in pipes.iter().enumerate() {
        for (x, pipe) in line.iter().enumerate() {
            if *pipe == Pipe::Start {
                return (x, y);
            }
        }
    }
    panic!("No start found")
}

fn are_connected(p1_port: &Port, p2: &Pipe) -> bool {
    p2.get_ports().contains(&p1_port.opposite())
}

fn get_distances(pipes: &Vec<Vec<Pipe>>, start_x: usize, start_y: usize) -> Vec<Vec<Option<u32>>> {
    let mut distance_field = vec![vec![Option::None; pipes[0].len()]; pipes.len()];

    let mut queue = queue![];
    queue.add((start_x, start_y));
    distance_field[start_y][start_x] = Some(0);

    while queue.size() > 0 {
        // println!("=====================");
        // println!("queue: {:?}", queue);
        // debug::print_distances(&distance_field);

        let (x, y) = queue.remove().unwrap();
        let pipe = &pipes[y][x];

        let distance = distance_field[y][x].unwrap();

        for port in pipe.get_ports() {
            let (neighbor_x, neighbor_y) = match port {
                Port::North => (x as i32, y as i32 - 1),
                Port::East => (x as i32 + 1, y as i32),
                Port::South => (x as i32, y as i32 + 1),
                Port::West => (x as i32 - 1, y as i32),
            };
            if neighbor_x < 0
                || neighbor_y < 0
                || neighbor_x >= pipes[0].len() as i32
                || neighbor_y >= pipes.len() as i32
            {
                continue;
            }
            let neighbor_x = neighbor_x as usize;
            let neighbor_y = neighbor_y as usize;

            if !are_connected(&port, &pipes[neighbor_y][neighbor_x]) {
                continue;
            }

            if let Some(_) = &distance_field[neighbor_y][neighbor_x] {
                continue;
            }
            distance_field[neighbor_y][neighbor_x] = Some(distance + 1);
            queue.add((neighbor_x, neighbor_y)).unwrap();
        }
    }

    distance_field
}

fn remove_unnecessary_pipes(
    original_pipes: &Vec<Vec<Pipe>>,
    distance_field: &Vec<Vec<Option<u32>>>,
) -> Vec<Vec<Pipe>> {
    let mut pipes = original_pipes.clone();

    for y in 0..pipes.len() {
        for x in 0..pipes[0].len() {
            if let None = distance_field[y][x] {
                pipes[y][x] = Pipe::Empty;
            }
        }
    }
    pipes
}

fn find_farthest_pipe_distance(input: &str) -> u32 {
    let pipes = parse_pipes(input);

    debug::print_pipes(&pipes);

    let (start_x, start_y) = find_start(&pipes);
    let distance_field = get_distances(&pipes, start_x, start_y);

    debug::print_pipes_connected_to_start(&pipes, &distance_field);
    debug::print_distances(&distance_field);

    distance_field
        .iter()
        .map(|line| line.iter().max().unwrap())
        .max()
        .unwrap()
        .unwrap()
}

fn expand_pipes(original_pipes: &Vec<Vec<Pipe>>) -> Vec<Vec<Pipe>>

fn find_number_of_inside_fields(input: &str) -> u32 {
    let pipes = parse_pipes(input);

    debug::print_pipes(&pipes);

    let (start_x, start_y) = find_start(&pipes);
    let distance_field = get_distances(&pipes, start_x, start_y);

    let pipes = remove_unnecessary_pipes(&pipes, &distance_field);

    debug::print_pipes_connected_to_start(&pipes, &distance_field);

    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn small_input() {
        // The easiest way to open the data is to include it into the generated binary.
        let input = include_str!("../input/small.txt");
        assert_eq!(find_farthest_pipe_distance(input), 4)
    }

    #[test]
    fn small_extended() {
        // The easiest way to open the data is to include it into the generated binary.
        let input = include_str!("../input/small_extended.txt");
        assert_eq!(find_farthest_pipe_distance(input), 8)
    }

    #[test]
    fn large_input() {
        // You can also read the file completely into memory
        let file = std::fs::read_to_string("input/big.txt").expect("Could not open input file");
        assert_eq!(find_farthest_pipe_distance(&file), 7173)
    }

    #[test]
    fn small_input_inside() {
        // The easiest way to open the data is to include it into the generated binary.
        let input = include_str!("../input/small.txt");
        assert_eq!(find_number_of_inside_fields(input), 0)
    }

    #[test]
    fn small_extended_inside() {
        // The easiest way to open the data is to include it into the generated binary.
        let input = include_str!("../input/small_extended.txt");
        assert_eq!(find_number_of_inside_fields(input), 0)
    }

    #[test]
    fn large_input_inside() {
        // You can also read the file completely into memory
        let file = std::fs::read_to_string("input/big.txt").expect("Could not open input file");
        assert_eq!(find_number_of_inside_fields(&file), 0)
    }
}
