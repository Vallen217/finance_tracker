use common::{file_pathing, utils, Date, Pathing};
use display::display::*;
use read_csv::CsvLines;
use std::{io, process::exit};
use write_csv::CsvFields;

pub mod common;
pub mod display;
pub mod read_csv;
pub mod write_csv;

fn main() {
    let date = Date::current_date();
    let path = Pathing::generate_file_path(&date, true).unwrap();
    let mut csv_lines = utils::instantiate_csv_lines(Some(path.month_path));

    // recurrent expenses
    let _ = file_pathing::empty_file(csv_lines.file_path.clone());
    if !file_pathing::empty_file(csv_lines.file_path.clone()) {
        // CsvLines::compile_csv(&mut csv_lines).unwrap();
        // let mut csv_fields = CsvLines::compile_re_exp(&mut csv_lines).unwrap();
        // CsvFields::push_re_exp(&mut csv_fields, csv_lines.file_path.clone());
    }

    // TODO:
    // write recurrent_exp.csv with with write_file
    // change display_previous_file pathing to ignore re_exp
    // write unit tests for re_exp

    loop {
        println!(
            "\n(mf)  - Modify the current file\
        \n(df)  - Display the current file\
        \n(dpf) - Display a previous file\
        \n(dre) - Display recurrent expenses\
        \n(q)   - Quit the program\
        \n\nOperation:"
        );

        let mut oper = String::new();
        io::stdin().read_line(&mut oper).unwrap();

        if oper.trim() == "mf" {
            println!(
                "\n\n(rl)  - Remove the last file entry\
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

        if oper.trim() == "dre" {
            let re_exp_path = format!(
                "{}/Documents/Finance/Records/recurrent_expenses.csv",
                file_pathing::user_path().unwrap()
            );
            if file_pathing::file_exists(re_exp_path.clone()) {
                let mut re_exp_lines = CsvLines {
                    file_path: re_exp_path.clone(),
                    lines: vec![],
                };

                CsvLines::compile_csv(&mut re_exp_lines).unwrap();
                let mut re_exp_fields = CsvLines::compile_re_exp(&mut re_exp_lines).unwrap();
                CsvFields::push_re_exp(&mut re_exp_fields, re_exp_lines.file_path.clone());

                display_file(re_exp_path);
                display_distr(re_exp_fields);
            } else {
                println!(
                    "Error: file not found: {}\nAborting operation.",
                    re_exp_path
                );
            }
        }

        if oper.trim() == "q" {
            println!("");
            exit(0);
        }
    }
}
