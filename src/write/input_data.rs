use super::*;
use crate::common::utils::csv_date_form;

#[allow(dead_code)]
impl CsvFields {
    pub fn get_new_data(&mut self) -> Result<(), Box<dyn Error>> {
        let date = csv_date_form();
        self.date.push(date);

        self.user_field_vals("Expense: ")?;
        // Skip commodity if expense is skipped.
        if self.expense.expense[self.expense.expense.len() - 1] != 0.0 {
            self.user_field_vals("Commodity: ")?;
        }
        self.user_field_vals("Income: ")?;

        let gross_expense = Self::calc_field_vals(self.expense.expense.clone());
        let gross_income = Self::calc_field_vals(self.income.clone());
        let net_income: f32 = gross_income - gross_expense;

        self.gross_expense.push(gross_expense);
        self.gross_income.push(gross_income);
        self.net_income.push(net_income);

        Ok(())
    }

    fn user_field_vals(&mut self, input_prompt: &str) -> Result<(), Box<dyn Error>> {
        println!("\n{}", input_prompt);
        let mut field_input = String::new();
        io::stdin().read_line(&mut field_input)?;
        let no_input: bool = field_input.trim().is_empty();

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
    use super::input_data::CsvFields;

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
