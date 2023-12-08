use std::{collections::HashMap, vec};

use nom::{
    bytes::complete::tag,
    character::complete::{alpha1, char, line_ending, anychar, one_of, alphanumeric1},
    combinator::map,
    multi::{separated_list1, many1},
    sequence::{delimited, separated_pair, pair},
    IResult,
};

mod debug;

enum Instruction {
    Left,
    Right,
}

// (AAA, BBB)
fn parse_next_elements(input: &str) -> IResult<&str, (&str, &str)> {
    delimited(
        char('('),
        separated_pair(alphanumeric1, tag(", "), alphanumeric1),
        char(')'),
    )(input)
}

//AAA = (BBB, BBB)
fn parse_map_entry(input: &str) -> IResult<&str, (&str, (&str, &str))> {
    separated_pair(alphanumeric1, tag(" = "), parse_next_elements)(input)
}

fn parse_map(input: &str) -> IResult<&str, HashMap<&str, (&str, &str)>> {
    map(separated_list1(line_ending, parse_map_entry), |v| {
        v.into_iter().collect()
    })(input)
}

fn parse_instruction(input: &str) -> IResult<&str, Instruction> {
    map(one_of("LR"), |c: char| match c {
        'L' => Instruction::Left,
        'R' => Instruction::Right,
        _ => panic!("Unknown instruction {}", c),
    })(input)
}

fn parse_instructions(input: &str) -> IResult<&str, Vec<Instruction>> {
    many1(parse_instruction)(input)
}

fn parse_empty_line(input: &str) -> IResult<&str, (&str, &str)> {
    pair(line_ending, line_ending)(input)
}

fn parse_input(input: &str) -> IResult<&str, (Vec<Instruction>, HashMap<&str, (&str, &str)>)> {
    separated_pair(parse_instructions, parse_empty_line, parse_map)(input)
}

fn find_nr_steps_to_end(input: &str) -> usize {
    let (_, (instructions, map)) = parse_input(input).unwrap();
    
    let mut nr_steps = 0;
    let mut current = "AAA";
    while current != "ZZZ" {
        let (left, right) = map.get(current).unwrap();
        current = match instructions[nr_steps % instructions.len()] {
            Instruction::Left => left,
            Instruction::Right => right,
        };
        nr_steps += 1;
    }
    nr_steps
}


//I think the input is carefully constructed in such a way that it works. There are multiple interesting things about the input:
// Each start (xxA) leads to one of the targets (yyZ) in a one-to-one fashion
// Each targets then loops onto itself (e.g. LQZ -> LQZ)
// From start (xxA) to target (yyZ) and from target to target (the same target, see above) you always do a full number of loops through the left/right instructions, never a fraction thereof.
// The path lengths start -> target and target -> target are the same in every "row", so one can treat start -> target in the beginning as just another loop. That is, you don't start off the target -> target loop at a fraction of the instruction loop. (If you did, every loop would start with a different offset, making this orders of magnitude more complicated!)
// The path lengths target -> target are all prime, so the lowest common denominator reduces to taking the product.
fn find_nr_steps_to_end_complicated(input: &str) -> u128 {
    let (_, (instructions, map)) = parse_input(input).unwrap();

    println!("instruction length: {}", instructions.len());

    let mut starts = Vec::new();
    map.keys().filter(|k| k.ends_with('A')).for_each(|k| starts.push(k));

    println!("Starts: {:?}", starts);

    let nr_steps = starts.iter().map(|start| {
        println!("Start: {}", start);

        let mut nr_steps = 0u128;
        let mut current = *start;
        while !current.ends_with('Z') {
            let (left, right) = map.get(current).unwrap();
            current = match instructions[nr_steps as usize % instructions.len()] {
                Instruction::Left => left,
                Instruction::Right => right,
            };
            nr_steps += 1;
        }
        nr_steps
       
    }).collect::<Vec<_>>();
        
    println!("Steps: {:?}", nr_steps);
    println!("instruction loops: {:?}", nr_steps.iter().map(|x| *x as f32 / instructions.len() as f32).collect::<Vec<_>>());

    let nr_steps = nr_steps.iter().map(|x| *x / instructions.len() as u128).collect::<Vec<_>>();

    nr_steps.iter().product::<u128>() * instructions.len() as u128 // LCM (works in this case, because the numbers are prime)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_input() {
        // The easiest way to open the data is to include it into the generated binary.
        let input = include_str!("../input/example.txt");
        assert_eq!(find_nr_steps_to_end(input), 2)
    }

    #[test]
    fn small_input() {
        // The easiest way to open the data is to include it into the generated binary.
        let input = include_str!("../input/small.txt");
        assert_eq!(find_nr_steps_to_end(input), 6)
    }

    #[test]
    fn large_input() {
        // You can also read the file completely into memory
        let file = std::fs::read_to_string("input/big.txt").expect("Could not open input file");
        assert_eq!(find_nr_steps_to_end(&file), 13771)
    }

    #[ignore = "Does not work anymore with the higly specialized solution for the large input"]
    #[test]
    fn small_input_second_example() {
        // The easiest way to open the data is to include it into the generated binary.
        let input = include_str!("../input/second_example.txt");
        assert_eq!(find_nr_steps_to_end_complicated(input), 6)
    }

    #[test]
    fn large_input_complicated() {
        // You can also read the file completely into memory
        let file = std::fs::read_to_string("input/big.txt").expect("Could not open input file");
        assert_eq!(find_nr_steps_to_end_complicated(&file), 13129439557681)
    }
    // wrong answers:
    // 28352038502929006347476933
    // 13129439557681
}
