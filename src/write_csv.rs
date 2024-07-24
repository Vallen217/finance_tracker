pub mod compile_input;
pub mod mod_file;
pub mod write_csv;

use crate::common::utils::csv_date_form;
use crate::display::display::display_file;
use regex::Regex;
use std::error::Error;
use std::io;

#[derive(Debug)]
pub struct CsvFields {
    pub date: Vec<String>,
    pub expense: ExpenseFields,
    pub gross_expense: Vec<f32>,
    pub income: Vec<f32>,
    pub gross_income: Vec<f32>,
    pub net_income: Vec<f32>,
}

#[derive(Debug, Clone)]
pub struct ExpenseFields {
    pub expense: Vec<f32>,
    pub commodity: Vec<String>,
}
