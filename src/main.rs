use common::utils;
use read_csv::CsvLines;
use write_csv::CsvFields;

pub mod common;
pub mod display;
pub mod read_csv;
pub mod write_csv;

fn main() {
    let file_path =
        String::from("/home/vallen/Workspace/finance_tracker/test_files/good_files/file_1.csv");
    let mut csv_lines = utils::instantiate_csv_lines(true);
    let mut csv_fields = utils::instantiate_csv_fields();

    let _ = CsvLines::compile_csv(&mut csv_lines, &mut csv_fields);
    let _ = CsvFields::compile_input(&mut csv_fields);
    let _ = CsvFields::write_csv(csv_fields, file_path);
    // dbg!("{:?}", &csv_lines);

    // TODO: add menu
    // let operation: string = io
    // add line
    // remove line
    // display file
    // aggregate expense commodities
}
