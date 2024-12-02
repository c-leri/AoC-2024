use core::panic;
use std::{fs::File, io::Read};

fn is_report_safe(report: &[i16]) -> bool {
    // Ascending or descending with no duplicates and steps of maximum 3
    report.is_sorted_by(|a, b| a < b && a + 3 >= *b)
        || report.is_sorted_by(|a, b| a > b && a - 3 <= *b)
}

/// Tries every possible version of this report
/// with one less element until one of them is safe
fn is_report_safe_with_problem_dampener(report: &[i16]) -> bool {
    // First check the normal report
    is_report_safe(report)
        || (0..report.len())
            .map(|i| {
                // Version of the report without the element of index i
                report[..i]
                    .iter()
                    .chain(&report[i + 1..report.len()])
                    .copied()
                    .collect::<Vec<_>>()
            })
            .any(|report_without_i| {
                // Check the safety of this report
                is_report_safe(&report_without_i)
            })
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
    let nb_safe_reports = reports.iter().fold(0, |acc, report| {
        if is_report_safe_with_problem_dampener(report) {
            acc + 1
        } else {
            acc
        }
    });

    println!("Result: {}", nb_safe_reports);
}
