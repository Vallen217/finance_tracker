pub mod file_pathing;
pub mod utils;

use chrono::Local;
use dirs;
use std::{error::Error, fs, io, path, process};

pub struct Pathing {
    pub year_path: String,
    pub month_path: String,
    pub day_path: String,
}
