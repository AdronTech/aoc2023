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

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum ExpandedPipe {
    Pipe(Pipe),
    Unknown,
    Inside,
    Outside,
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

fn expand_pipes(original_pipes: &Vec<Vec<Pipe>>) -> Vec<Vec<ExpandedPipe>> {
    let mut expanded_pipes =
        vec![vec![ExpandedPipe::Unknown; original_pipes[0].len() * 3]; original_pipes.len() * 3];

    for y in 0..original_pipes.len() {
        for x in 0..original_pipes[0].len() {
            let pipe = &original_pipes[y][x];
            let (x, y) = (x * 3, y * 3);

            if let Pipe::Empty = pipe {
                continue;
            }

            expanded_pipes[y + 1][x + 1] = ExpandedPipe::Pipe(pipe.clone());

            for port in pipe.get_ports() {
                let (offset_x, offset_y) = match port {
                    Port::North => (1, 0),
                    Port::East => (2, 1),
                    Port::South => (1, 2),
                    Port::West => (0, 1),
                };
                expanded_pipes[y + offset_y][x + offset_x] = ExpandedPipe::Pipe(pipe.clone());
            }
        }
    }

    expanded_pipes
}

fn reduce_pipes(expanded_pipes: &mut Vec<Vec<ExpandedPipe>>) -> Vec<Vec<ExpandedPipe>> {
    let mut reduced_pipes = vec![vec![ExpandedPipe::Unknown; expanded_pipes[0].len() / 3]; expanded_pipes.len() / 3];
    for y in 0..expanded_pipes.len() / 3 {
        for x in 0..expanded_pipes[0].len() /3 {
            let (x_expanded, y_expanded) = (x * 3, y * 3);
            reduced_pipes[y][x] = expanded_pipes[y_expanded + 1][x_expanded + 1].clone();
        }
    }
    reduced_pipes
}

fn flood_outside(expanded_pipes: &mut Vec<Vec<ExpandedPipe>>) {
    let mut queue = queue![];

    for y in 0..expanded_pipes.len() {
        queue.add((0, y)).unwrap();
        queue.add((expanded_pipes[0].len() - 1, y)).unwrap();
    }

    for x in 0..expanded_pipes[0].len() {
        queue.add((x, 0)).unwrap();
        queue.add((x, expanded_pipes.len() - 1)).unwrap();
    }

    while queue.size() > 0 {
        let (x, y) = queue.remove().unwrap();
        if let ExpandedPipe::Unknown = expanded_pipes[y][x] {
            expanded_pipes[y][x] = ExpandedPipe::Outside;

            for (offset_x, offset_y) in [(1, 0), (0, 1), (-1, 0), (0, -1)].iter() {
                let (neighbor_x, neighbor_y) = (x as i32 + offset_x, y as i32 + offset_y);
                if neighbor_x < 0
                    || neighbor_y < 0
                    || neighbor_x >= expanded_pipes[0].len() as i32
                    || neighbor_y >= expanded_pipes.len() as i32
                {
                    continue;
                }
                let neighbor_x = neighbor_x as usize;
                let neighbor_y = neighbor_y as usize;

                if let ExpandedPipe::Unknown = expanded_pipes[neighbor_y][neighbor_x] {
                    queue.add((neighbor_x, neighbor_y)).unwrap();
                }
            }
        }
    }
}

fn convert_unknown_to_inside(expanded_pipes: &mut Vec<Vec<ExpandedPipe>>) {
    for y in 0..expanded_pipes.len() {
        for x in 0..expanded_pipes[0].len() {
            if let ExpandedPipe::Unknown = expanded_pipes[y][x] {
                expanded_pipes[y][x] = ExpandedPipe::Inside;
            }
        }
    }
}   

fn find_number_of_inside_fields(input: &str) -> u32 {
    let pipes = parse_pipes(input);

    debug::print_pipes(&pipes);

    let (start_x, start_y) = find_start(&pipes);
    let distance_field = get_distances(&pipes, start_x, start_y);

    let pipes = remove_unnecessary_pipes(&pipes, &distance_field);

    debug::print_pipes_connected_to_start(&pipes, &distance_field);

    let mut expanded_pipes = expand_pipes(&pipes);
    debug::print_expanded_pipes(&expanded_pipes);

    flood_outside(&mut expanded_pipes);
    convert_unknown_to_inside(&mut expanded_pipes);
    debug::print_expanded_pipes(&expanded_pipes);

    let mut reduced_pipes = reduce_pipes(&mut expanded_pipes);
    debug::print_reduced_pipes(&reduced_pipes);

    reduced_pipes.iter().flatten().filter(|p| **p == ExpandedPipe::Inside).count() as u32
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
    fn small_enclosed_input_inside() {
        // The easiest way to open the data is to include it into the generated binary.
        let input = include_str!("../input/small_enclosed.txt");
        assert_eq!(find_number_of_inside_fields(input), 4)
    }

    #[test]
    fn small_enclosed2_input_inside() {
        // The easiest way to open the data is to include it into the generated binary.
        let input = include_str!("../input/small_enclosed2.txt");
        assert_eq!(find_number_of_inside_fields(input), 8)
    }

    #[test]
    fn small_enclosed3_input_inside() {
        // The easiest way to open the data is to include it into the generated binary.
        let input = include_str!("../input/small_enclosed3.txt");
        assert_eq!(find_number_of_inside_fields(input), 10)
    }

    #[test]
    fn large_input_inside() {
        // You can also read the file completely into memory
        let file = std::fs::read_to_string("input/big.txt").expect("Could not open input file");
        assert_eq!(find_number_of_inside_fields(&file), 291)
    }
}
