use core::panic;
use std::{fs::File, io::Read};

use regex::Regex;

const MUL_ARGS_RE: &str = r"mul\(([0-9]{1,3}),([0-9]{1,3})\)";
const DONT_DO_RE: &str = r"don't\(\)[\S\s]*?(do\(\)|$)";

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

fn extract_outside_of_dont_do(input: &str) -> Vec<&str> {
    // Regex that matches a substring starting with "dont't()"
    // and ending with either "do()" or the end of the string
    let re = Regex::new(DONT_DO_RE).unwrap();

    // Return the substrings NOT matched by the regex
    re.split(input).collect()
}

fn process(input: &str) -> u32 {
    extract_outside_of_dont_do(input)
        .iter()
        .flat_map(|substr| {
            // Get the args of each mul of each substring
            extract_mul_args(substr)
                .into_iter()
                // Calculate the multiplications
                .map(|args| args[0] * args[1])
        })
        // Sum everything together
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
        let input = "xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))";

        let expected_result = 48;

        assert_eq!(process(input), expected_result);
    }

    #[test]
    fn test_multiple_dont_do() {
        let input = "xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))";

        let expected_result = 88;

        assert_eq!(process(input), expected_result);
    }

    #[test]
    fn test_dont_without_do() {
        let input = "xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))don't()_mul(5,5)";

        let expected_result = 88;

        assert_eq!(process(input), expected_result);
    }
}
