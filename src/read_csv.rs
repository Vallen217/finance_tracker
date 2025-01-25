pub mod compile_csv;
pub mod compile_re_exp;

use super::write_csv::{CsvFields, ExpenseFields};
#[allow(unused_imports)]
use crate::common::utils;
use csv::{self, StringRecord};
use std::error::Error;

#[derive(Debug)]
pub struct CsvLines {
    pub file_path: String,
    pub lines: Vec<StringRecord>,
}

// utility functions
pub fn str_to_f32(mut str_val: &str) -> f32 {
    if str_val.contains("$") {
        str_val = &str_val[1..];
    }

    let val: f32 = match str_val.parse() {
        Ok(num) => num,
        Err(err) => {
            dbg!("{:?}", err);
            panic!("Error: parsing: {}", str_val);
        }
    };

    val
}

pub fn compile_expense_field(expense: &mut ExpenseFields, str_val: &str) {
    let mut expense_vec: Vec<&str> = vec![];
    for val in str_val.split(":") {
        expense_vec.push(val.trim());
    }

    let expense_val_str = &expense_vec[0][1..expense_vec[0].len()];
    let expense_val = str_to_f32(expense_val_str);
    let commodity_val = if expense_vec[1].is_empty() {
        String::from("")
    } else {
        String::from(&expense_vec[1][0..(expense_vec[1].len())])
    };
    expense.expense.push(expense_val);
    expense.commodity.push(commodity_val);
}
