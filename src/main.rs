use core::panic;
use std::{fs::File, io::Read};

use regex::Regex;

const MUL_ARGS_RE: &str = r"mul\(([0-9]{1,3}),([0-9]{1,3})\)";

fn extract_mul_args(input: &str) -> Vec<[u32; 2]> {
    // Regex that matches the arguments of a mul
    let re = Regex::new(MUL_ARGS_RE).unwrap();

    re
        // Match all the mul expressions in the input
        .captures_iter(input)
        // Extract the args of each mul
        .map(|c| c.extract())
        // Parse each arg
        .map(|(expr, args)| {
            args.map(|arg| {
                arg.parse().unwrap_or_else(|e| {
                    panic!("Unable to parse the args of expression {}: {:?}", expr, e)
                })
            })
        })
        .collect()
}

fn process(input: &str) -> u32 {
    let muls_args = extract_mul_args(input);

    muls_args
        .iter()
        // Calculate the multiplications
        .map(|args| args[0] * args[1])
        // Sum all the results
        .sum()
}

fn main() {
    let mut input_file = File::open("./input.txt").expect("Unable to open input.txt file");

    let mut input_string = String::new();
    input_file
        .read_to_string(&mut input_string)
        .expect("Unable to read input file");

    let result = process(&input_string);
    println!("Result: {}", result);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let input = "xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))";

        let expected_result = 161;

        assert_eq!(process(input), expected_result);
    }
}
