use super::*;

#[allow(dead_code)]
impl CsvFields {
    pub fn write_csv(&mut self, file_path: String) -> Result<(), Box<dyn Error>> {
        // TODO: Clean the csv formatting
        let mut csv_write = csv::Writer::from_path(&file_path)?;
        let e_fields_pad =
            longest_commodity(self.expense.expense.clone(), self.expense.commodity.clone());
        let mut i_field_pad = longest_field_len(self.income.clone());

        if i_field_pad < "Income".len() {
            i_field_pad = "Income".len();
        }

        csv_write.write_record(&[
            "Date",
            "       Expense",
            format!(
                "{}{}",
                pad("gross expense", e_fields_pad + 6),
                "Gross Expense"
            )
            .as_str(),
            " Income",
            format!("{}{}", pad("gross income", i_field_pad + 6), "Gross Income").as_str(),
            " Net Income",
        ])?;

        for i in 0..self.date.len() {
            let e_fields = format!(
                "${:.2}: {}",
                self.expense.expense[i], self.expense.commodity[i]
            );
            let g_e_field = format!("${:.2}", self.gross_expense[i]);
            let i_field = format!("${:.2}", self.income[i]);
            let g_i_field = format!("${:.2}", self.gross_income[i]);
            let n_i_field = format!("${:.2}", self.net_income[i]);

            csv_write.write_record(&[
                self.date[i].clone(),
                format!(" {}", e_fields),
                format!("{}{}", pad(&e_fields, e_fields_pad), g_e_field),
                format!("{}{}", pad(&g_e_field, 14), i_field),
                format!("{}{}", pad(&i_field, i_field_pad), g_i_field),
                format!("{}{}", pad(&g_i_field, 13), n_i_field),
            ])?;
            csv_write.flush()?;
        }

        display_file(file_path);
        Ok(())
    }
}

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
