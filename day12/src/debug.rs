use colored::*;

use crate::{Status, StatusRange};

pub fn print_row_monochrome(row: &Vec<Status>) {
    row.iter().for_each(|s| match s {
        Status::Damaged => print!("{}", "#"),
        Status::Unknown => print!("{}", "?"),
        Status::Operational => print!("{}", "."),
    });
}

pub fn print_row(row: &Vec<Status>) {
    row.iter().for_each(|s| match s {
        Status::Damaged => print!("{}", "#".red()),
        Status::Unknown => print!("{}", "?".yellow()),
        Status::Operational => print!("{}", ".".green()),
    });
}

pub fn print_row_range(row_range: &Vec<StatusRange>) {
    row_range.iter().for_each(|r| match r {
        StatusRange::ExactlyDamaged(n) => print!("{},", format!("{}", n).red()),
        StatusRange::OneOrMoreOperational => print!("{},", "1".green()),
        StatusRange::ZeroOrMoreOperational => print!("{},", "0".green()),
    });
}

pub fn print_report(row_prefix: &Vec<Status>, row: &Vec<Status>, row_range: &Vec<StatusRange>) {
    print_row_monochrome(row_prefix);
    print_row(row);
    print!(" ");
    print_row_range(row_range);
}
