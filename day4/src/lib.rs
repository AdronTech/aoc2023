mod debug;

fn parse_card(input: &str) -> (Vec<u32>, Vec<u32>) {
    input
        .split_once(':')
        .map(|(card, numbers)| numbers)
        .unwrap()
        .split_once('|')
        .map(|(winning_numbers, numbers_you_have)| {
            (
                winning_numbers
                    .split_whitespace()
                    .map(|n| n.trim().parse::<u32>().unwrap())
                    .collect::<Vec<u32>>(),
                numbers_you_have
                    .split_whitespace()
                    .map(|n| n.trim().parse::<u32>().unwrap())
                    .collect::<Vec<u32>>(),
            )
        })
        .unwrap()
}

fn score_card(number_of_matching_cards: u32) -> u32 {
    if number_of_matching_cards == 0 {
        return 0;
    }

    1 * 2u32.pow(number_of_matching_cards - 1)
}

fn number_of_matching_numbers(winning_numbers: &[u32], numbers_you_have: &[u32]) -> u32 {
    winning_numbers
        .iter()
        .filter(|n| numbers_you_have.contains(n))
        .count() as u32
}

fn calc_score_of_all_cards(input: &str) -> u32 {
    input.lines().map(|line| {
        let (winning_numbers, numbers_you_have) = parse_card(line);
        let score = score_card(number_of_matching_numbers(&winning_numbers, &numbers_you_have));
        println!("{} => {}", line, score);
        score 
    }).sum()
}

fn calc_accumulated_score_of_all_cards(input: &str) -> u32 {
    // create an array num_cards of length input.lines().count() with all values set to 1
    let mut num_cards = vec![1; input.lines().count()];

    input.lines().enumerate().for_each(|(i, line)| {
        let (winning_numbers, numbers_you_have) = parse_card(line);
        let number_of_matching_numbers = number_of_matching_numbers(&winning_numbers, &numbers_you_have);
        for j in 0..number_of_matching_numbers as usize {
            num_cards[i + j + 1] += num_cards[i];
        }
    });

    println!("{:?}", num_cards);

    num_cards.iter().sum()
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn small_input() {
        // The easiest way to open the data is to include it into the generated binary.
        let input = include_str!("../input/small.txt");
        assert_eq!(calc_score_of_all_cards(input), 13)
    }

    #[test]
    fn large_input() {
        // You can also read the file completely into memory
        let file = std::fs::read_to_string("input/big.txt").expect("Could not open input file");
        assert_eq!(calc_score_of_all_cards(&file), 26426)
    }

    #[test]
    fn small_input_power_sum() {
        // The easiest way to open the data is to include it into the generated binary.
        let input = include_str!("../input/small.txt");
        assert_eq!(calc_accumulated_score_of_all_cards(input), 30)
    }

    #[test]
    fn large_input_power_sum() {
        // You can also read the file completely into memory
        let file = std::fs::read_to_string("input/big.txt").expect("Could not open input file");
        assert_eq!(calc_accumulated_score_of_all_cards(&file), 6227972)
    }
}
