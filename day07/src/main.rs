use std::cmp::Ordering;

#[derive(Debug)]
#[derive(PartialEq, Eq)]
enum HandType{
    Five,
    Four,
    Full,
    Three,
    TwoPair,
    Pair,
    High,
}
impl PartialOrd for HandType {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}
impl Ord for HandType {
    fn cmp(&self, other: &Self) -> Ordering {
        // println!("HT: {:?} {:?}", self, other);
        // self.cards.cmp(&other.cards)
        if *self == *other {
            return Ordering::Equal;
        }
        if *self == HandType::Five {
            return Ordering::Greater;
        }
        if *other == HandType::Five {
            return Ordering::Less;
        }
        if *self == HandType::Four {
            return Ordering::Greater;
        }
        if *other == HandType::Four {
            return Ordering::Less;
        }
        if *self == HandType::Full {
            return Ordering::Greater;
        }
        if *other == HandType::Full {
            return Ordering::Less;
        }
        if *self == HandType::Three {
            return Ordering::Greater;
        }
        if *other == HandType::Three {
            return Ordering::Less;
        }
        if *self == HandType::TwoPair {
            return Ordering::Greater;
        }
        if *other == HandType::TwoPair {
            return Ordering::Less;
        }
        if *self == HandType::Pair {
            return Ordering::Greater;
        }
        if *other == HandType::Pair {
            return Ordering::Less;
        }
        return Ordering::Equal;
        // match (self, other) {
        //     (a, b) if a == b => Equal,
        //
        // }
    }
}
// impl PartialEq for HandType {
//     fn eq(&self, other: &Self) -> bool {
//         println!("eq");
//         self == other
//     }
// }
// impl Eq for HandType {}

#[derive(Debug)]
struct Hand {
    cards : Vec<i32>,
    hand_type : HandType,
    bid : i32
}
impl Hand {
    pub fn new(cards : Vec<i32>, bid : i32) -> Self {
        let mut sorted_cards = cards.clone();
        sorted_cards.sort();
        let mut duplicates : Vec<i32> = vec!();
        let mut curr = sorted_cards[0];
        let mut count = 0;
        let mut num_jokers = 0;
        for c in sorted_cards {
            if c == 0 {
                num_jokers += 1;
            }
            else{
                if c == curr {
                    count += 1;
                }
                else {
                    duplicates.push(count);
                    curr = c;
                    count = 1;
                }
            }
        }
        duplicates.push(count);
        duplicates.sort_by(|a, b| b.cmp(a));
        let mut hand_type = HandType::High;
        // println!{"{:?}", duplicates};
        for dup in duplicates {
            let mut dup = dup;
            dup += num_jokers;
            num_jokers = 0;
            if dup == 5 {
                hand_type = HandType::Five;
            }
            if dup == 4 {
                hand_type = HandType::Four;
            }
            if dup == 3 {
                hand_type = match hand_type {
                    HandType::Pair => HandType::Full,
                    _ => HandType::Three
                };
            }
            if dup == 2 {
                hand_type = match hand_type {
                    HandType::Pair => HandType::TwoPair,
                    HandType::Three => HandType::Full,
                    _ => HandType::Pair
                };
            }
        }
        Self { cards : cards, hand_type : hand_type, bid: bid}
    }
}
impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}
impl Ord for Hand {
    fn cmp(&self, other: &Self) -> Ordering {
        // println!("{:?} {:?}", self, other);
        // self.cards.cmp(&other.cards)
        match self.hand_type.cmp(&other.hand_type) {
            Ordering::Equal => {
                for i in 0..self.cards.len(){
                    if self.cards[i] > other.cards[i] {
                        return Ordering::Greater;
                    }
                    if self.cards[i] < other.cards[i] {
                        return Ordering::Less;
                    }
                }
                return Ordering::Equal;
            }
            o => {return o;},
        }
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
        let cards = words[0].chars().map(|x| {
            match x {
                'A' => 14,
                'K' => 13,
                'Q' => 12,
                'J' => 11,
                'T' => 10,
                x => x as i32 - '0' as i32,
            }
        }).collect();
        hands.push(Hand::new(cards, words[1].parse().unwrap()));
    }
    hands
}

fn parse_hands_2(file_name : &str) -> Vec<Hand>{
    let contents = std::fs::read_to_string(file_name).unwrap();
    let mut hands = vec!();
    for line in contents.lines() {
        let words = line.split(" ").collect::<Vec<&str>>();
        let cards = words[0].chars().map(|x| {
            match x {
                'A' => 14,
                'K' => 13,
                'Q' => 12,
                'J' => 0,
                'T' => 10,
                x => x as i32 - '0' as i32,
            }
        }).collect();
        hands.push(Hand::new(cards, words[1].parse().unwrap()));
    }
    hands
}

fn part1(mut hands : Vec<Hand>) -> i32 {
    // hands.sort_by(|a, b| b.cmp(a));
    hands.sort();
    // println!("{:?}", hands);
    let mut acc : i32 = 0;
    for (i, hand) in hands.iter().enumerate() {
        acc += TryInto::<i32>::try_into(i+1).unwrap() * hand.bid;
    }
    acc
}

fn part2(mut hands : Vec<Hand>) -> i32 {
    hands.sort();
    // println!("{:?}", hands);
    let mut acc : i32 = 0;
    for (i, hand) in hands.iter().enumerate() {
        acc += TryInto::<i32>::try_into(i+1).unwrap() * hand.bid;
    }
    acc
}

fn main() {
    // let hands = parse_hands("src/hands.txt");
    let hands = parse_hands("src/hands.txt");
    let result_1 = part1(hands);
    println!("Part 1: {}", result_1);
    let hands_2 = parse_hands_2("src/hands.txt");
    let result_2 = part2(hands_2);
    println!("Part 2: {}", result_2);
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn part_1_test() {
        let hands = parse_hands("src/hands_test.txt");
        let result = part1(hands);
        assert_eq!(result, 6440);
    }
    #[test]
    fn part_2_test() {
        let hands = parse_hands_2("src/hands_test.txt");
        let result = part2(hands);
        assert_eq!(result, 5905);
    }
}
