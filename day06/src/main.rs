
#[derive(Debug)]
struct Race {
    time : i64,
    distance : i64,
}

fn parse_races(file_name : &str) -> Vec<Race> {
    let contents = std::fs::read_to_string(file_name).unwrap();
    let lines = contents.lines().collect::<Vec<&str>>();
    let time_line = lines[0];
    let distance_line = lines[1];
    let times : Vec<i64> = time_line.split(" ").filter(|s|{
        match s.parse::<i64>() {
            Err(_) => false,
            Ok(_) => true,
        }
    }).map(|s| {
        s.parse::<i64>().unwrap()
    }).collect();
    let distances : Vec<i64> = distance_line.split(" ").filter(|s|{
        match s.parse::<i64>() {
            Err(_) => false,
            Ok(_) => true,
        }
    }).map(|s| {
        s.parse::<i64>().unwrap()
    }).collect();

    let mut races = vec!();
    for i in 0..times.len(){
        let race = Race{time : times[i], distance : distances[i]};
        races.push(race);
    }
    races
}

fn parse_race(file_name : &str) -> Race {
    let contents = std::fs::read_to_string(file_name).unwrap();
    let lines = contents.lines().collect::<Vec<&str>>();
    let time_line = lines[0];
    let distance_line = lines[1];
    let time : i64 = time_line.chars().fold(0, |acc, curr| {
        match curr.to_digit(10) {
            Some(i) => acc * 10 + i as i64,
            _ => acc,
        }
    });
    let distance : i64 = distance_line.chars().fold(0, |acc, curr| {
        match curr.to_digit(10) {
            Some(i) => acc * 10 + i as i64,
            _ => acc,
        }
    });
    Race{time : time, distance : distance}
}

fn number_of_ways_to_beat(race : Race) -> i64 {
    let mut acc = 0;
    for i in 0..=race.time {
        let speed = i;
        let remaining_time = race.time - i;
        let my_distance = speed*remaining_time;
        if my_distance > race.distance {
            acc += 1;
        }
    }
    acc
}

fn part1(races : Vec<Race>) -> i64 {
    let mut acc = 1;
    for race in races {
        let num = number_of_ways_to_beat(race);
        acc *= num;
    }
    acc
}


fn part2(race : Race) -> i64 {
    number_of_ways_to_beat(race)
}

fn main() {
    let races = parse_races("src/races.txt");
    let result1 = part1(races);
    println!("Part 1: {}", result1);
    let race = parse_race("src/races.txt");
    let result2 = part2(race);
    println!("Part 2: {}", result2);
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn part1_test() {
        let races = parse_races("src/races_test.txt");
        let result = part1(races);
        assert_eq!(result, 288);
    }
    #[test]
    fn part2_test() {
        let race = parse_race("src/races_test.txt");
        let result = part2(race);
        assert_eq!(result, 71503);
    }
}
