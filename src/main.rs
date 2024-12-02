use core::panic;
use std::{fs::File, io::Read};

fn is_report_safe(report: &[i16]) -> bool {
    // Ascending or descending with no duplicates
    (report.is_sorted_by(|a, b| a < b) || report.is_sorted_by(|a, b| a > b))
        && !report
            .windows(2)
            // No steps of more than 3
            .any(|w| w[0] < w[1] - 3 || w[0] > w[1] + 3)
}

fn main() {
    // ================================
    //            READ INPUT
    // ================================
    let mut input_file = File::open("./input.txt").expect("Unable to open input.txt file");

    let mut input_string = String::new();
    input_file
        .read_to_string(&mut input_string)
        .expect("Unable to read input file");

    // ================================
    //           PARSE INPUT
    // ================================
    let reports: Vec<Vec<i16>> = input_string
        .lines()
        .enumerate()
        .map(|(i, line)| {
            line.split(" ")
                .map(|num| {
                    num.parse().unwrap_or_else(|e| {
                        panic!("Unable to parse number {} at line {}: {:?}", num, i, e)
                    })
                })
                .collect()
        })
        .collect();

    // ================================
    //        COUNT SAFE REPORTS
    // ================================
    let nb_safe_reports =
        reports.iter().fold(
            0,
            |acc, report| if is_report_safe(report) { acc + 1 } else { acc },
        );

    println!("Result: {}", nb_safe_reports);
}
