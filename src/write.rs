pub mod input_data;
pub mod write_data;

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
