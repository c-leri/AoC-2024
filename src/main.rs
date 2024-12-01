use std::{
    cmp::{max, min},
    fs::File,
    io::Read,
};

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
    let mut columns: Vec<Vec<u32>> = vec![vec![], vec![]];

    for (i, line) in input_text.lines().enumerate() {
        let nums_str: Vec<&str> = line.split("   ").collect();

        for (j, column) in columns.iter_mut().enumerate() {
            let num_str = nums_str
                .get(j)
                .unwrap_or_else(|| panic!("Unable to read left number at line {}", i));

            let num = num_str.parse().unwrap_or_else(|e| {
                panic!(
                    "Unable to parse left number {} at line {}: {:?}",
                    num_str, i, e
                )
            });

            column.push(num);
        }
    }

    // ================================
    //           SORT LISTS
    // ================================
    columns.iter_mut().for_each(|column| column.sort());

    // ================================
    //        CALCULATE DISTANCE
    // ================================
    let mut result: u32 = 0;

    for (i, left_num) in columns
        .first()
        .expect("Couldn't get left column ???")
        .iter()
        .enumerate()
    {
        let right_num = columns
            .get(1)
            .expect("Couldn't get right column ???")
            .get(i)
            .unwrap_or_else(|| panic!("Couldn't retrieve number at line {}", i));

        result += max(*left_num, *right_num) - min(*left_num, *right_num);
    }

    println!("Result: {}", result);
}
