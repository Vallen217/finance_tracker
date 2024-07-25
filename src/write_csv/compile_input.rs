use super::*;

impl CsvFields {
    pub fn compile_input(&mut self, path: String) -> Result<(), Box<dyn Error>> {
        let date = csv_date_form();
        self.date.push(date);

        self.input_csv_field("Expense: ", &path)?;
        // Skip commodity if expense is skipped.
        if self.expense.expense[self.expense.expense.len() - 1] != 0.0 {
            self.input_csv_field("Commodity: ", &path)?;
        }
        self.input_csv_field("Income: ", &path)?;

        let g_expense = Self::calc_field_vals(self.expense.expense.clone());
        let g_income = Self::calc_field_vals(self.income.clone());
        let n_income: f32 = g_income - g_expense;

        self.gross_expense.push(g_expense);
        self.gross_income.push(g_income);
        self.net_income.push(n_income);

        self.write_csv(path)?;
        // TODO: return mod_file
        Ok(())
    }

    fn input_csv_field(&mut self, input_prompt: &str, path: &String) -> Result<(), Box<dyn Error>> {
        println!("\n{}", input_prompt);
        let mut field_input = String::new();
        io::stdin().read_line(&mut field_input)?;
        let no_input: bool = field_input.trim().is_empty();

        if field_input.contains("-") {
            println!("Error: Invalid input: {}", field_input);
            let sync_fields = self.date.len();
            if self.expense.expense.len() != sync_fields {
                self.expense.expense.pop();
            }
            if self.expense.commodity.len() != sync_fields {
                self.expense.commodity.pop();
            }
            if self.income.len() != sync_fields {
                self.income.pop();
            }
            return self.compile_input(path.to_owned());
        }

        match input_prompt {
            "Expense: " => {
                let field_val: f32 = if no_input {
                    0.0
                } else {
                    field_input.trim().parse()?
                };

                self.expense.expense.push(field_val);
            }
            "Commodity: " => {
                let field_val = String::from(field_input.trim());
                self.expense.commodity.push(field_val);
            }
            "Income: " => {
                let field_val: f32 = if no_input {
                    0.0
                } else {
                    field_input.trim().parse()?
                };

                self.income.push(field_val);
            }
            _ => {
                dbg!("{:?}", input_prompt);
                panic!("This shouldn't happen");
            }
        }

        Ok(())
    }

    fn calc_field_vals(field_vals: Vec<f32>) -> f32 {
        let mut agg_vals: f32 = 0.0;
        for val in field_vals {
            agg_vals += val;
        }
        agg_vals
    }
}

#[cfg(test)]
mod unit_tests {
    use super::compile_input::CsvFields;

    #[test]
    fn test_calc_field_vals() {
        let vec_1: Vec<f32> = vec![37.09, 27.98, 65.52];
        let mut vec_2: Vec<f32> = vec![124.07, 0.0, 90.83, 12.34];
        let expected_result_1: f32 = 130.59;
        let expected_result_2: f32 = 357.83;

        let result_1 = CsvFields::calc_field_vals(vec_1);
        assert_eq!(expected_result_1, result_1);

        vec_2.push(result_1);
        let result_2 = CsvFields::calc_field_vals(vec_2);
        assert_eq!(expected_result_2, result_2);
    }
}
