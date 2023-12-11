use std::collections::HashMap;

// struct NetworkNode {
// }

fn parse_file(file_name : &str) -> (Vec<bool>, HashMap<String, (String,String)>) {
    let contents = std::fs::read_to_string(file_name).unwrap();
    let sections : Vec<&str> = contents.split("\n\n").collect();
    let instructions_line = sections[0];
    let instructions = instructions_line.chars().map(|c| {
        match c {
            'R' => true,
            _ => false,
        }
    }).collect();
    let nodes_lines: Vec<&str> = sections[1].lines().collect();
    let mut nodes = HashMap::new();
    for node_line in nodes_lines {
        let node_data = parse_node_line(node_line);
        nodes.insert(node_data.0, (node_data.1, node_data.2));
    }
    // println!("{:?}", nodes);
    (instructions, nodes)
}

fn parse_node_line(node_line : &str) -> (String, String, String){
    // let curr = node_line[..3];
    // let left = node_line[..3];
    // let right = node_line[..3];
    let curr = node_line.chars().take(3).collect();
    let left = node_line.chars().skip(7).take(3).collect();
    let right = node_line.chars().skip(12).take(3).collect();
    (curr, left, right)
}

fn number_of_step_needed(instructions: Vec<bool>, nodes: HashMap<String, (String,String)>) -> i32 {
    let mut count = 0;
    let mut curr = "AAA".to_string();
    let mut i = 0;
    while !curr.eq("ZZZ") {
        count += 1;
        let instr = instructions[i];
        i += 1;
        if i >= instructions.len() {
            i = 0;
        }

        if instr {
            curr = nodes.get(&curr).unwrap().1.clone();
        }
        else {
            curr = nodes.get(&curr).unwrap().0.clone();
        }
    }
    count
}

fn number_of_step_needed_ghost(instructions: Vec<bool>, nodes: HashMap<String, (String,String)>) -> i32 {
    let mut count = 0;
    let mut curr_nodes : Vec<String> = vec!();
    for k in nodes.keys() {
        if k.chars().last().unwrap() == 'A' {
            curr_nodes.push(k.clone());
        }
    }
    // curr_nodes.push("AAA".to_string());
    println!("{}", curr_nodes.len());
    let mut i = 0;
    while !are_all_ended(&curr_nodes) {
        println!("{} {:?}", count, curr_nodes);
        count += 1;
        let instr = instructions[i];
        i += 1;
        if i >= instructions.len() {
            i = 0;
        }

        for curr_nodes_index in 0..curr_nodes.len() {
            if instr {
                curr_nodes[curr_nodes_index] = nodes.get(&curr_nodes[curr_nodes_index]).unwrap().1.clone();
            }
            else {
                curr_nodes[curr_nodes_index] = nodes.get(&curr_nodes[curr_nodes_index]).unwrap().0.clone();
            }
        }

    }
    count
}

fn are_all_ended(curr_nodes : &Vec<String>) -> bool {
    for cn in curr_nodes {
        if cn.chars().last().unwrap() != 'Z' {
            return false;
        }
    }
    true
}

fn part_1(instructions: Vec<bool>, nodes: HashMap<String, (String,String)>) -> i32 {
    number_of_step_needed(instructions, nodes)
}
fn part_2(instructions: Vec<bool>, nodes: HashMap<String, (String,String)>) -> i32 {
    number_of_step_needed_ghost(instructions, nodes)
}

fn main() {
    let (instructions, nodes) = parse_file("src/network.txt");
    let result_1 = part_1(instructions, nodes);
    println!("Part 1: {}", result_1);
    let (instructions, nodes) = parse_file("src/network.txt");
    let result_2 = part_2(instructions, nodes);
    println!("Part 2: {}", result_2);
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn part_1_test_1() {
        let (instructions, nodes) = parse_file("src/network_test_1.txt");
        let result = part_1(instructions, nodes);
        assert_eq!(result, 2);
    }
    #[test]
    fn part_1_test_2() {
        let (instructions, nodes) = parse_file("src/network_test_2.txt");
        let result = part_1(instructions, nodes);
        assert_eq!(result, 6);
    }
    #[test]
    fn part_2_test_3() {
        let (instructions, nodes) = parse_file("src/network_test_3.txt");
        let result = part_2(instructions, nodes);
        assert_eq!(result, 6);
    }
}
