fn read_to_grid(file_name : String) -> Vec<Vec<char>> {
    std::fs::read_to_string(file_name).unwrap().lines().map(|x| x.chars().collect::<Vec<char>>()).collect::<Vec<Vec<char>>>()
}

fn is_touching_symbol(y: usize, number_range : (usize, usize), grid : &Vec<Vec<char>>) -> bool {
    let mut left = 0;
    if number_range.0 > 0 {
        left = number_range.0-1;
    }
    let mut right = grid[y].len();
    if number_range.1 < grid[y].len() {
        right = number_range.1 + 1
    }
    let mut top = 0;
    if y>0 {
        top = y-1;
    }
    let mut bottom = grid.len();
    if y+1 < grid.len(){
        bottom = y+1+1;
    }
    for iy in top..bottom {
        for ix in left..right {
            let character = grid[iy][ix];
            if !(character <= '9' && character >= '0') && character != '.' {
                return true;
            }
        }
    }
    return false;
}

fn id_from_range(y : usize, number_range : (usize, usize), grid : &Vec<Vec<char>>) -> i32 {
    let mut num = 0;
    for i in number_range.0..number_range.1 {
        num *= 10;
        num += grid[y][i] as i32 - '0' as i32;
    }
    num
}

fn get_part_number_sum(grid : &Vec<Vec<char>>, ) -> i32 {
    let mut sum = 0;
    for (y, row) in grid.iter().enumerate() {
        let mut number_ranges : Vec<(usize, usize)> = vec!();
        let mut curr_start = None;
        //get the index range of all the numbers in the row
        for (x, character) in row.iter().enumerate() {
            if character <= &'9' && character >= &'0' {
                if curr_start == None{
                    curr_start = Some(x);
                }
            }
            else {
                if curr_start != None {
                    number_ranges.push((curr_start.unwrap(), x));
                    curr_start = None;
                }
            }
            // print!("{}", character);
        }
        if curr_start != None {
            number_ranges.push((curr_start.unwrap(), row.len()));
            curr_start = None;
        }
        // println!(" {:?}", number_ranges);
        //check for each index range if it is next to a symbol{
        for number_range in number_ranges {
            if is_touching_symbol(y, number_range, grid){
                let id = id_from_range(y, number_range, grid);
                // println!("{}", id);
                sum += id;
            }
        }

    }
    sum
}

fn part1(file_name : String) -> i32 {
    let grid = read_to_grid(file_name);
    // for row in grid.iter() {
    //     println!("{:?}", row);
    // }
    get_part_number_sum(&grid)
}

fn main() {
    // let p1 = part1(String::from("src/engine_schematic_test.txt"));
    // let p1 = part1(String::from("src/es_test_2.txt"));
    let p1 = part1(String::from("src/engine_schematic.txt"));
    println!("Part 1: {}", p1);
}
