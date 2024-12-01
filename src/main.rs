use std::{collections::HashMap, fs::File, io::Read};

fn main() {
    // ================================
    //            READ INPUT
    // ================================
    let mut input_file = File::open("./input.txt").expect("Missing input.txt file");

    let mut input_text = String::new();
    input_file
        .read_to_string(&mut input_text)
        .expect("Unable to read input file");

    // ================================
    //           PARSE INPUT
    // ================================
    let mut left_column: Vec<u32> = vec![];
    let mut right_column_appearances: HashMap<u32, u32> = HashMap::new();

    for (i, line) in input_text.lines().enumerate() {
        let nums: Vec<u32> = line
            .split("   ")
            .map(|num_str| {
                num_str.parse::<u32>().unwrap_or_else(|e| {
                    panic!("Unable to parse number {} at line {}: {:?}", num_str, i, e)
                })
            })
            .collect();

        let left_num = *nums.first().unwrap();
        left_column.push(left_num);

        let right_num = *nums.get(1).unwrap();
        if let Some(appearances) = right_column_appearances.get_mut(&right_num) {
            *appearances += 1;
        } else {
            right_column_appearances.insert(right_num, 1);
        }
    }

    // ================================
    //         CALCULATE RESULT
    // ================================
    let result = left_column.iter().copied().fold(0, |acc, num| {
        acc + num * right_column_appearances.get(&num).unwrap_or(&0)
    });

    println!("Result: {}", result);
}
