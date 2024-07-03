use super::*;

#[allow(dead_code)]
impl CsvFields {
    pub fn write_csv(self, file_path: String) -> Result<(), Box<dyn Error>> {
        let mut csv_write = csv::Writer::from_path(&file_path)?;

        csv_write.write_record(&[
            "Date ",
            "Expense ",
            "Gross Expense ",
            "Income ",
            "Gross Expense ",
            "Net Expense ",
        ])?;

        for i in 0..self.date.len() {
            csv_write.write_record(&[
                self.date[i].clone(),
                format!(
                    " ${:.2}: {}",
                    self.expense.expense[i], self.expense.commodity[i]
                ),
                format!(" ${:.2}", self.gross_expense[i]),
                format!(" ${:.2}", self.income[i]),
                format!(" ${:.2}", self.gross_income[i]),
                format!(" ${:.2}", self.net_income[i]),
            ])?;
            csv_write.flush()?;
        }

        Ok(())
    }

    fn delete_last_line(&mut self) -> Result<(), Box<dyn Error>> {
        self.date.pop();
        self.expense.expense.pop();
        self.expense.commodity.pop();
        self.gross_expense.pop();
        self.income.pop();
        self.gross_income.pop();
        self.net_income.pop();

        Ok(())
    }
}
