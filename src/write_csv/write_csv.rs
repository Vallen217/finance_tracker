use super::*;

#[allow(dead_code)]
impl CsvFields {
    pub fn write_csv(&mut self, file_path: String) -> Result<(), Box<dyn Error>> {
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

        Ok(())
    }

    pub fn push_re_exp(&mut self, path: String) {
        // pushes csv values equal to the number of recurrent expense entries due.
        let mut iter = self.date.len() - self.income.len();
        while iter != 0 {
            // recurrent expense csv file doesn't include income.
            // an empty value must be pushed to prevent indexing errors.
            self.income.push(0.0);

            // by default the entire CsvField.expense.expense vec would be used
            // to get the gross_expense. resulting in something like this:
            // Expense,                 Gross Expense,
            // $0.00: ,                 $0.00,
            // $12.65: Spotify Premium, $56.65,
            // $44.00: CCW Safe,        $56.65,
            //
            // however the gross_expense should be calculated iteratively from each expense.
            // resulting like this:
            // Expense,                 Gross Expense,
            // $0.00: ,                 $0.00,
            // $12.65: Spotify Premium, $12.65,
            // $44.00: CCW Safe,        $56.65,
            let g_expense =
                calc_field_vals(self.expense.expense.clone()[0..self.income.len()].to_vec());
            let g_income = calc_field_vals(self.income.clone());
            let n_income: f32 = g_income - g_expense;

            self.gross_expense.push(g_expense);
            self.gross_income.push(g_income);
            self.net_income.push(n_income);

            iter -= 1;
        }
        self.write_csv(path).unwrap();
    }
}
