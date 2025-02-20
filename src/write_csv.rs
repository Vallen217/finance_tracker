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

// utility functions
pub fn pad(word: &str, char_len: usize) -> String {
    let spaces = char_len - word.len();
    let padding = " ".repeat(spaces);
    padding
}

pub fn longest_field_len(field: Vec<f32>) -> usize {
    let mut sorted_vec = field.clone();
    sorted_vec.sort_by(|a, b| a.partial_cmp(b).unwrap());
    let longest_ele_len = sorted_vec[field.len() - 1].to_string().len();
    longest_ele_len + 5
}

pub fn longest_commodity(expense: Vec<f32>, commodity: Vec<String>) -> usize {
    let mut longest_iter: usize = 0;
    for i in 0..expense.len() {
        let e_fields = format!("${:.2}: {}", expense[i], commodity[i]);
        if e_fields.len() > longest_iter {
            longest_iter = e_fields.len();
        }
    }

    longest_iter + 1
}

pub fn calc_field_vals(field_vals: Vec<f32>) -> f32 {
    let mut agg_vals: f32 = 0.0;
    for val in field_vals {
        agg_vals += val;
    }
    agg_vals
}
