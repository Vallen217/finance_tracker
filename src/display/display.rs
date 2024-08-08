use super::*;
use crate::write_csv;
use std::collections::HashMap;

pub fn display_file(path: String) {
    match fs::read_to_string(&path) {
        Ok(file) => println!("\n{}", file),
        Err(_) => {
            println!("Error: unable to read '{}'", path);
            crate::main();
        }
    }
}

pub fn expense_distr(csv_fields: write_csv::CsvFields) {
    let mut values: HashMap<&String, f32> = HashMap::new();

    for (i, val) in csv_fields.expense.commodity.iter().enumerate() {
        // Initialize novel key value pairs
        if !values.contains_key(val) {
            values.insert(val, csv_fields.expense.expense[i]);
        } else {
            // Increment existing key's value
            let key_val: f32 = *values.get(val).unwrap();
            values.insert(val, key_val + csv_fields.expense.expense[i]);
        }
    }

    // Remove blank key
    if values.contains_key(&String::from("")) {
        values.remove(&String::from(""));
    }

    println!("Aggregated Expense Distribution");
    let mut newline = 0;
    for (key, val) in values.iter() {
        newline += 1;
        if newline == 4 {
            newline = 0;
            println!("{}: ${:.2}, ", key, val);
        } else {
            print!("{}: ${:.2}, ", key, val);
        }
    }

    newline = 1;
    let g_e = csv_fields.gross_expense.last().unwrap_or(&0.0);
    let g_i = csv_fields.gross_income.last().unwrap_or(&0.0);

    print!(
        "\nPercentile Income Distribution\nProfit: {:.2}%, ",
        (100.0 / (g_e + g_i) * g_i)
    );
    for (key, val) in values.iter() {
        newline += 1;

        let per_val = (100.0 / (g_e + g_i)) * val;

        if newline == 4 {
            newline = 0;
            println!("{}: {:.2}%, ", key, per_val);
        } else {
            print!("{}: {:.2}%, ", key, per_val);
        }
    }
    println!("");
}
