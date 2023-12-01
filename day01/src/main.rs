fn file_to_lines (file_name : String) -> Vec<String> {
    let lines : Vec<String> = std::fs::read_to_string(file_name).unwrap().lines().map(|x| x.to_string()).collect();
    return lines;
}

fn lines_to_calibrations_1 (lines: Vec<String>) -> Vec<i32> {
    let mut calibrations = vec!();
    for line in lines{
        let mut first = -1;
        let mut last = -1;
        for c in line.chars() {
            if c >= '0' && c <= '9' {
                if first == -1 {
                    first = c as i32 - '0' as i32;
                }
                last = c as i32 - '0' as i32;
            }
        }
        calibrations.push(10*first + last);
    }
    return calibrations;
}

fn lines_to_calibrations_2 (lines: Vec<String>) -> Vec<i32> {
    let spelled_out = vec![("one",1),("two",2),("three",3),("four",4),("five",5),("six",6),("seven",7),("eight",8),("nine",9),];
    let mut calibrations = vec!();
    for line in lines{
        let mut first = -1;
        let mut last = -1;
        for i in 0..line.len() {
            let c = line.chars().nth(i).unwrap();
            if c >= '0' && c <= '9' {
                if first == -1 {
                    first = c as i32 - '0' as i32;
                }
                last = c as i32 - '0' as i32;
            }
            else {
                for s in &spelled_out {
                    if s.0.len() + i <= line.len(){
                        let mut is_match = true;
                        for j in 0..s.0.len() {
                            if !(s.0.chars().nth(j) == line.chars().nth(j+i) ) {
                                is_match = false;
                            }
                        }
                        if is_match {
                            if first == -1 {
                                first = s.1;
                            }
                            last = s.1;
                        }

                    }
                }
            }
        }
        calibrations.push(10*first + last);
    }
    return calibrations;
}

fn part1 (file_name : String) -> i32 {
    let calibrations = lines_to_calibrations_1(file_to_lines(file_name));
    calibrations.iter().fold(0, |sum, x| sum+x)
}

fn part2 (file_name : String) -> i32 {
    let calibrations = lines_to_calibrations_2(file_to_lines(file_name));
    calibrations.iter().fold(0, |sum, x| sum+x)
}

fn main() {
    println!("part 1: {}", part1(String::from("src/calibration_document.txt")));
    println!("part 2: {}", part2(String::from("src/calibration_document.txt")));
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn sample_input_1() {
        let result = part1(String::from("src/calibration_document_test.txt"));
        assert_eq!(result, 142);
    }
    #[test]
    fn sample_input_2() {
        let result = part2(String::from("src/calibration_document_test_2.txt"));
        assert_eq!(result, 281);
    }
}
