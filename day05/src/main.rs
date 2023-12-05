#[derive(Debug)]
struct NumberMapping {
    ranges : Vec<(i64,i64,i64)>,
}

impl NumberMapping {
    fn apply_mapping(&self, seed : i64) -> i64 {
        for range in self.ranges.iter() {
            if seed >= range.1 && seed < range.1+range.2 {
                return seed - range.1 + range.0;
            }
        }
        seed
    }
}


fn parse_seed_maps(file_name : &str) -> (Vec<i64>, Vec<NumberMapping>){
    let contents = std::fs::read_to_string(file_name).unwrap();
    let sections  : Vec<&str> = contents.split("\n\n").collect();
    // println!("{:?}", sections[0]);
    let seeds = sections[0].split(": ").collect::<Vec<&str>>()[1].split(" ").collect::<Vec<&str>>().into_iter().map(|x| x.parse::<i64>().unwrap()).collect::<Vec<i64>>();
    // let seeds = vec!();
    let mut number_maps = vec!();
    for i in 1..sections.len() {
        let number_map = parse_section(sections[i]);
        // println!("{}",i);
        number_maps.push(number_map);
    }
    // println!("{:?}", seeds);
    // println!("{:?}", number_maps);
    // (vec!(), vec!())
    (seeds, number_maps)
}

fn parse_section(section : &str) -> NumberMapping {
    let mut ranges = vec!();
    let lines = section.lines().collect::<Vec<&str>>();
    for i in 1..lines.len() {
        let line = lines[i];
        let numbers = line.split(" ").collect::<Vec<&str>>().into_iter().map(|x| x.parse::<i64>().unwrap()).collect::<Vec<i64>>();
        ranges.push((numbers[0],numbers[1],numbers[2],));
    }
    NumberMapping {ranges : ranges}
}

fn part1(file_name : &str) -> i64 {
    let (seeds, number_mappings) = parse_seed_maps(file_name);

    let locations = seeds.into_iter().map(|s| {
        let mut acc : i64 = s;
        for nm in number_mappings.iter() {
            // println!("{}", acc);
            acc = nm.apply_mapping(acc);
        }
        acc

        // number_mappings.iter().fold(s, |acc : i32, curr : &NumberMapping| {
        //     let res = curr.apply_mapping(acc);
        //     res
        // })
    }).collect::<Vec<i64>>();
    // println!("{:?}", locations);
    locations.into_iter().min().unwrap()
}

fn main() {
    println!("Part 1: {}", part1("src/seed_maps.txt"));
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn part1_test() {
        let result = part1("src/seed_maps_test.txt");
        assert_eq!(result, 35);
    }
}
