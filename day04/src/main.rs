#[derive(Debug)]
struct Card {
    winning_numbers : Vec<i32>,
    your_numbers : Vec<i32>,
}

fn parse_cards(file_name : &str) -> Vec<Card> {
    let contents = std::fs::read_to_string(file_name).unwrap();
    let lines : Vec<&str> = contents.lines().collect();
    let mut cards = vec!();
    for line in lines{
        cards.push(parse_card(line));
    }
    return cards;
}

fn parse_card(line : &str) -> Card{
    let lists = line.split(": ").collect::<Vec<&str>>()[1].split(" | ").collect::<Vec<&str>>();
    Card{winning_numbers: parse_number_list(lists[0]), your_numbers: parse_number_list(lists[1])}
}

fn parse_number_list(list : &str) -> Vec<i32> {
    let mut nl : Vec<&str> = list.split(" ").collect::<Vec<&str>>();
    nl.retain(|x| {
        match *x {
            "" => false,
            _ => true,
        }
    });
    nl.into_iter().map(|x| x.parse::<i32>().unwrap()).collect::<Vec<i32>>()
}

fn number_of_matches (card: Card) -> i32 {
    let mut matches = 0;
    for your in card.your_numbers.iter(){
        for winning in card.winning_numbers.iter(){
            if your == winning {
                matches += 1;
            }
        }
    }
    matches
}

fn part1(file_name : &str) -> i32 {
    let cards = parse_cards(file_name);
    let mut sum = 0;
    for card in cards {
        let num = number_of_matches(card);
        sum += match num {
            0 => 0,
            n => 2_i32.pow((n - 1).try_into().unwrap()),
        };
    }
    sum
}

fn part2(file_name : &str) -> i32 {
    let cards = parse_cards(file_name);
    let cards_len = cards.len();
    let mut copies : Vec<i32> = vec![1; cards_len];
    for (index, card) in cards.into_iter().enumerate() {
        let num_matches = number_of_matches(card);
        let upper_bound = std::cmp::min(cards_len, index+1+num_matches as usize);
        for i in (index+1)..upper_bound{
            copies[i] += copies[index];
        }
    }
    copies.into_iter().fold(0, |sum, x| sum + x)
}

fn main() {
    println!("Part 1: {}", part1("src/cards.txt"));
    println!("Part 2: {}", part2("src/cards.txt"));
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn part1_test() {
        let result = part1("src/cards_test.txt");
        assert_eq!(result, 13);
    }
    #[test]
    fn part2_test() {
        let result = part2("src/cards_test.txt");
        assert_eq!(result, 30);
    }
}
