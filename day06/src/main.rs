
#[derive(Debug)]
struct Race {
    time : i32,
    distance : i32,
}

fn parse_races(file_name : &str) -> Vec<Race> {
    let contents = std::fs::read_to_string(file_name).unwrap();
    let lines = contents.lines().collect::<Vec<&str>>();
    let time_line = lines[0];
    let distance_line = lines[1];
    let times : Vec<i32> = time_line.split(" ").filter(|s|{
        match s.parse::<i32>() {
            Err(_) => false,
            Ok(_) => true,
        }
    }).map(|s| {
        s.parse::<i32>().unwrap()
    }).collect();
    let distances : Vec<i32> = distance_line.split(" ").filter(|s|{
        match s.parse::<i32>() {
            Err(_) => false,
            Ok(_) => true,
        }
    }).map(|s| {
        s.parse::<i32>().unwrap()
    }).collect();

    let mut races = vec!();
    for i in 0..times.len(){
        let race = Race{time : times[i], distance : distances[i]};
        races.push(race);
    }
    races
}

fn number_of_ways_to_beat(race : Race) -> i32 {
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

fn part1(races : Vec<Race>) -> i32 {
    let mut acc = 1;
    for race in races {
        let num = number_of_ways_to_beat(race);
        acc *= num;
    }
    acc
}

fn main() {
    println!("Hello, world!");
    let races = parse_races("src/races.txt");
    let result1 = part1(races);
    println!("Part 1: {}", result1);
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_works() {
        let races = parse_races("src/races_test.txt");
        let result = part1(races);
        assert_eq!(result, 288);
    }
}
