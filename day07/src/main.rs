use std::cmp::Ordering;

#[derive(Debug)]
struct Hand {
    cards : Vec<i32>,
    bet : i32,
}
impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}
impl Ord for Hand {
    fn cmp(&self, other: &Self) -> Ordering {
        self.cards.cmp(&other.cards)
    }
}
impl PartialEq for Hand {
    fn eq(&self, other: &Self) -> bool {
        self.cards == other.cards
    }
}
impl Eq for Hand {}

fn parse_hands(file_name : &str) -> Vec<Hand>{

    let contents = std::fs::read_to_string(file_name).unwrap();
    let mut hands = vec!();
    for line in contents.lines() {
        let words = line.split(" ").collect::<Vec<&str>>();
        hands.push(Hand{cards : words[0].chars().map(|x| {
            match x {
                'A' => 14,
                'K' => 13,
                'Q' => 12,
                'J' => 11,
                x => x as i32 - 0 as i32,
            }
        }).collect(), bet : words[1].parse().unwrap()})
    }
    // for line in contents.lines() {
    //
    //
    //     let cards Vec<i32> = line.chars.take(5).collect();
    // }
    hands
}

fn main() {
    // let hands = parse_hands("src/hands.txt");
    let hands = parse_hands("src/hands_test.txt");
    println!("{:?}", hands);
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn part1_test() {
        let hands = parse_hands("src/hands_test.txt");
        println!("{:?}", hands);

        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
