fn parse_file(file_name : String) -> Vec<(i32, Vec<(i32,i32,i32)>)> {
    let lines : Vec<String> = std::fs::read_to_string(file_name).unwrap().lines().map(|x| x.to_string()).collect();
    let mut games = vec!();

    for line in lines{
        games.push(parse_line(line));
    }

    games
}

fn parse_line(line: String) -> (i32, Vec<(i32,i32,i32)>) {
    let mut game = vec!();
    let colon_index = line.chars().position(|x| x ==':').unwrap();
    let id = line.chars().skip(5).take(colon_index - 5).collect::<String>().parse::<i32>().unwrap();
    // println!("{}", line.chars().skip(5).take(colon_index - 5).collect::<String>().parse::<i32>().unwrap());

    let just_rounds : String = line.chars().skip(2 + colon_index).collect();

    let rounds : Vec<String> = just_rounds.split("; ").map(|x| x.to_string()).collect();
    for round in rounds {
        let handfull : Vec<String> = round.split(", ").map(|x| x.to_string()).collect();
        game.push(parse_handfull(handfull));
    }

    (id, game)
}

fn parse_handfull(handfull : Vec<String>) -> (i32, i32, i32) {
    let mut nums = (0,0,0);
    for group in handfull {
        let index = group.chars().position(|x| x == ' ').unwrap();
        let num = group.chars().take(index).collect::<String>().parse::<i32>().unwrap();
        let color : String = group.chars().skip(index+1).collect();
        match &color as &str {
            "red" => {nums.0 += num},
            "green" => {nums.1 += num},
            "blue" => {nums.2 += num},
            _ => {},
        }
    }

    nums
}

fn get_valid_id_sum (games: Vec<(i32, Vec<(i32,i32,i32)>)>, max_values : (i32, i32, i32)) -> i32 {
    let mut valid_id_sum = 0;
    for g in games {
        let mut valid = true;
        for h in g.1 {
            if !(h.0 <= max_values.0 && h.1 <= max_values.1 && h.2 <= max_values.2) {
                valid = false;
            }
        }
        if valid {
            valid_id_sum += g.0;
        }
    }
    valid_id_sum
}

fn get_power_sum (games: Vec<(i32, Vec<(i32,i32,i32)>)>) -> i32 {
    let mut power_sum = 0;
    for g in games {
        let mut fewest_nums = (0,0,0);
        for h in g.1 {
            if h.0 > fewest_nums.0 {
                fewest_nums.0 = h.0;
            }
            if h.1 > fewest_nums.1 {
                fewest_nums.1 = h.1;
            }
            if h.2 > fewest_nums.2 {
                fewest_nums.2 = h.2;
            }
        }
        let curr_power = fewest_nums.0 * fewest_nums.1 * fewest_nums.2;
        power_sum += curr_power;
    }
    power_sum
}


fn part1(file_name : String) -> i32{
    let games = parse_file(file_name);
    get_valid_id_sum(games, (12, 13, 14))
}

fn part2(file_name : String) -> i32 {
    let games = parse_file(file_name);
    get_power_sum(games)
}

fn main() {
    let p1 = part1(String::from("src/games.txt"));
    println!("part1: {}", p1);
    let p2 = part2(String::from("src/games.txt"));
    println!("part1: {}", p2);
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn sample_input_part_1() {
        let result = part1(String::from("src/games_test.txt"));
        assert_eq!(result, 8);
    }
    #[test]
    fn sample_input_part_2() {
        let result = part2(String::from("src/games_test.txt"));
        assert_eq!(result, 2286);
    }
}
