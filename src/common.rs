pub mod file_pathing;
pub mod utils;

use crate::{
    read_csv::CsvLines,
    write_csv::{CsvFields, ExpenseFields},
};
use chrono::Local;
use dirs;
use std::collections::HashMap;
use std::env;
use std::{error::Error, fs, io, path, process};

#[derive(Debug)]
pub struct Pathing {
    pub year_path: String,
    pub month_path: String,
}

#[derive(Debug)]
pub struct Date {
    pub year: i16,
    pub month: i16,
    pub day: i16,
}
