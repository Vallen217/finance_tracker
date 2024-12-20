use common::{utils, Date, Pathing};
use display::display::*;
use read_csv::CsvLines;
use std::{io, process::exit};

pub mod common;
pub mod display;
pub mod read_csv;
pub mod write_csv;
fn main() {
    let date = Date::current_date();
    let path = Pathing::generate_file_path(&date, true).unwrap();
    let mut csv_lines = utils::instantiate_csv_lines(Some(path.month_path));

    loop {
        println!(
            "\n(mf)  - Modify the current file\
        \n(df)  - Display the current file\
        \n(dpf) - Display a previous file\
        \n(q)   - Quit the program\
        \n\nOperation:"
        );

        let mut oper = String::new();
        io::stdin().read_line(&mut oper).unwrap();

        if oper.trim() == "mf" {
            println!(
                "\n\n(rl#)  - Remove the last # file entry\
                \n(q)    - Quit file modification\
                \nPress any key to continue"
            );

            let csv_fields = CsvLines::compile_csv(&mut csv_lines).unwrap();
            write_csv::mod_file::mod_file(csv_fields, csv_lines.file_path.clone());
        }

        if oper.trim() == "df" {
            display_file(csv_lines.file_path.clone());
            let csv_fields = CsvLines::compile_csv(&mut csv_lines).unwrap();
            display_distr(csv_fields)
        }

        if oper.trim() == "dpf" {
            display_previous_file();
        }

        // TODO: reoccuring expenses
        // make a seperate csv file with expense fields and manually input dates
        // iterate through the dates
        // check if all reo_expense date < current date exist in the current file
        // insert all missing entries
        // // if the date matches the current date
        // input it into the primary csv file

        if oper.trim() == "q" {
            println!("");
            exit(0);
        }
    }
}
