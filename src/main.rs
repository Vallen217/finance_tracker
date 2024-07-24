use common::utils;
use display::display::display_file;
use read_csv::CsvLines;
use std::{io, process::exit};
use write_csv::CsvFields;

pub mod common;
pub mod display;
pub mod read_csv;
pub mod write_csv;

fn main() {
    let path =
        String::from("/home/vallen/Workspace/finance_tracker/test_files/good_files/file_3.csv");
    // let pathing = Pathing::generate_file_path(&Date::current_date(), true).unwrap();
    let mut csv_lines = utils::instantiate_csv_lines(Some(path.clone()));
    let mut csv_fields = CsvLines::compile_csv(&mut csv_lines).unwrap();
    let _ = CsvFields::write_csv(&mut csv_fields, path);

    // TODO:
    // display previous file
    // display aggregated expense commodities

    println!(
        "\n\n(mf) - Modify the current file\
        \n(df) - Display the current file\
        \n(q)  - Quit the program\
        \n\nOperation:"
    );

    loop {
        let mut oper = String::new();
        io::stdin().read_line(&mut oper).unwrap();

        if oper.trim() == "mf" {
            println!(
                "\n\n(rl#)  - Remove the last # file entry\
                \n(rlq#) - Remove the last # file entry and quit\
                \n(q)    - Quit the loop\
                \nPress any key to continue"
            );

            let csv_fields = CsvLines::compile_csv(&mut csv_lines).unwrap();
            return write_csv::mod_file::mod_file(csv_fields, csv_lines.file_path.clone());
        }

        if oper.trim() == "df" {
            display_file(csv_lines.file_path.clone());
        }

        if oper.trim() == "q" {
            println!("");
            exit(0);
        }
    }
}
