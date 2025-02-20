use super::*;
use crate::common::file_pathing;
use crate::read_csv::CsvLines;

pub fn display_file(path: String) {
    match fs::read_to_string(&path) {
        Ok(file) => println!("\n{}", file),
        Err(_) => {
            println!("Error: unable to read '{}'", path);
            crate::main();
        }
    }
}

pub fn display_re_exp(path: String, re_exp_fields: write_csv::CsvFields) {
    display_file(path);

    let g_e = write_csv::calc_field_vals(re_exp_fields.expense.expense);

    println!("Gross Recurrent Expenses:\n${:.2}", g_e);
}

pub fn display_previous_file() {
    let user_dir = file_pathing::user_path().unwrap();
    let parent_dir = format!("{}/Documents/Finance/Records", user_dir);
    let year_dir = file_pathing::user_input_pathing(parent_dir, "year");
    let month_file = file_pathing::user_input_pathing(year_dir.unwrap(), "month").unwrap();

    display_file(month_file.clone());

    let mut csv_lines = utils::instantiate_csv_lines(Some(month_file));
    let csv_fields = CsvLines::compile_csv(&mut csv_lines).unwrap();

    display_distr(csv_fields)
}

pub fn display_distr(csv_fields: write_csv::CsvFields) {
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

    // Display values in descending order
    let mut desc_vals: Vec<f32> = vec![];
    for val in values.values() {
        desc_vals.push(val.clone());
    }
    desc_vals.sort_by(|a, b| b.partial_cmp(a).unwrap());

    println!("Aggregated Expense Values");
    let mut newline = 0;
    for (i, val) in desc_vals.iter().enumerate() {
        let key = utils::get_value_key(&desc_vals[i], &values);
        newline += 1;

        if newline == 4 {
            newline = 0;
            println!("{}: ${:.2}, ", key, val);
        } else {
            print!("{}: ${:.2}, ", key, val);
        }
    }

    newline = 1;
    let g_i = csv_fields.gross_income.last().unwrap_or(&0.0);
    let n_i = csv_fields.net_income.last().unwrap_or(&0.0);

    print!(
        "\n\nIncome Distribution\nProfit: {:.2}%, ",
        (100.0 / (g_i / n_i))
    );
    for (i, val) in desc_vals.iter().enumerate() {
        let per_key = utils::get_value_key(&desc_vals[i], &values);
        newline += 1;

        let per_val = (100.0 / (g_i)) * val;

        if newline == 4 {
            newline = 0;
            println!("{}: {:.2}%, ", per_key, per_val);
        } else {
            print!("{}: {:.2}%, ", per_key, per_val);
        }
    }
    println!("");
}
