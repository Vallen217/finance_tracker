pub mod read_data;

use super::write::{CsvFields, ExpenseFields};
use crate::common::utils;
use csv::{self, StringRecord};
use std::error::Error;

#[derive(Debug)]
pub struct CsvLines {
    pub file_path: String,
    pub lines: Vec<StringRecord>,
}
