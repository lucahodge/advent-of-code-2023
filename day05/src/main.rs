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
    fn apply_to_range(&self, seed_range : (i64, i64)) -> Vec<(i64, i64)> {
        let mut srs = vec!();
        srs.push(seed_range);
        //break up
        for mr in self.ranges.iter() {
            let mut new_srs : Vec<(i64, i64)> = vec!();
            for sr in srs.iter() {
                //TODO: break up the seed range if it is interupted by a mapping range
                new_srs.append(&mut new_ranges(*sr, (mr.1, mr.2)));
                // if range_overlap(){
                //
                // }
                // else {
                //     new_srs.push(*sr);
                // }
            }
            srs = new_srs;
        }
        //apply
        let mut res = vec!();
        for sr in srs {
            res.push((self.apply_mapping(sr.0), sr.1));
        }
        res
    }
}

fn new_ranges(seed_range : (i64,i64), source_range : (i64,i64)) -> Vec<(i64,i64)> {
    //TODO: 
    let mut res = vec!();

    let front_source_in : bool = source_range.0 > seed_range.0 && source_range.0 < seed_range.0+seed_range.1;
    let back_source_in : bool = source_range.0+source_range.1 > seed_range.0 && source_range.0+source_range.1 < seed_range.0+seed_range.1;

    if front_source_in && back_source_in {
        //TODO: 
    }
    else if front_source_in {
        //TODO: 
    }
    else if back_source_in {
        //TODO: 
    }
    else {
        res.push(seed_range);
    }
    res
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

fn seed_ranges_from_seed_list(seed_ranges : Vec<i64>) -> Vec<(i64, i64)> {
    let mut seeds = vec!();
    for i in (0..seed_ranges.len()).step_by(2) {
        let start = seed_ranges[i];
        let number = seed_ranges[i+1];
        seeds.push((start, number));
    }
    seeds
}

fn apply_all_mappings_range(range: (i64, i64), number_mappings: &Vec<NumberMapping>) -> Vec<i64> {
    let mut ranges = vec!();
    ranges.push(range);
    for nm in number_mappings {
        let mut new_ranges : Vec<(i64, i64)> = vec!();
        for range in ranges {
            new_ranges.append(&mut nm.apply_to_range(range));
        }
        ranges = new_ranges;
    }
    ranges.into_iter().map(|x| x.0).collect()
}

fn part2(file_name : &str) -> i64 {
    let (seed_list, number_mappings) = parse_seed_maps(file_name);
    let seed_ranges = seed_ranges_from_seed_list(seed_list);
    
    let mut locations : Vec<i64> = vec!();
    for seed_range in seed_ranges {
        locations.append(&mut apply_all_mappings_range(seed_range, &number_mappings));
    }
    locations.into_iter().min().unwrap_or(-1)
}

fn main() {
    println!("Part 1: {}", part1("src/seed_maps.txt"));
    println!("Part 2: {}", part2("src/seed_maps.txt"));
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn part1_test() {
        let result = part1("src/seed_maps_test.txt");
        assert_eq!(result, 35);
    }
    #[test]
    fn part2_test() {
        let result = part2("src/seed_maps_test.txt");
        assert_eq!(result, 46);
    }
}
