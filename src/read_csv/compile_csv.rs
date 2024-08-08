use super::*;

impl CsvLines {
    pub fn compile_csv(&mut self) -> Result<CsvFields, Box<dyn Error>> {
        self.lines.clear();

        let csv_reader = csv::ReaderBuilder::new()
            .has_headers(true)
            .from_path(self.file_path.clone());

        for result in csv_reader?.records() {
            let record = result?;
            self.lines.push(record);
        }

        let csv_fields = self.parse_csv_line(utils::instantiate_csv_fields());
        Ok(csv_fields)
    }

    fn parse_csv_line(&mut self, mut csv_fields: CsvFields) -> CsvFields {
        for line in &self.lines {
            let mut iter = 0;
            for val in line {
                let str_val = val.trim();

                match iter {
                    0 => csv_fields.date.push(str_val.to_string()),
                    1 => compile_expense_field(&mut csv_fields.expense, str_val),
                    2 => csv_fields.gross_expense.push(str_to_f32(str_val)),
                    3 => csv_fields.income.push(str_to_f32(str_val)),
                    4 => csv_fields.gross_income.push(str_to_f32(str_val)),
                    5 => csv_fields.net_income.push(str_to_f32(str_val)),
                    _ => {
                        dbg!("{:?}", iter);
                        panic!("Error: iterator out of bounds");
                    }
                }
                iter += 1;
            }
        }

        csv_fields
    }
}

fn str_to_f32(mut str_val: &str) -> f32 {
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

fn compile_expense_field(expense: &mut ExpenseFields, str_val: &str) {
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

#[cfg(test)]
mod unit_tests {
    use super::*;

    #[test]
    fn test_str_to_f32() {
        let test_str: &str = "$50.41";
        let expected_result: f32 = 50.41;
        let result = str_to_f32(test_str);

        assert_eq!(result, expected_result);
    }

    #[test]
    fn test_expense_fields() {
        let mut csv_lines = utils::instantiate_csv_lines(None);
        let csv_fields = CsvLines::compile_csv(&mut csv_lines).unwrap();

        let expected_expense_2: f32 = 65.52;
        let expected_expense_5: f32 = 90.83;
        let expected_commodity_3 = String::from("Loan");
        let expected_commodity_4 = String::from("");
        // dbg!("{:?}", &csv_fields);

        assert_eq!(expected_expense_2, csv_fields.expense.expense[2]);
        assert_eq!(expected_expense_5, csv_fields.expense.expense[5]);
        assert_eq!(expected_commodity_3, csv_fields.expense.commodity[3]);
        assert_eq!(expected_commodity_4, csv_fields.expense.commodity[4]);
    }

    #[test]
    fn test_csv_fields() {
        let mut csv_lines = utils::instantiate_csv_lines(None);
        let csv_fields = CsvLines::compile_csv(&mut csv_lines).unwrap();

        let expected_date_0 = String::from("2024-06-07");
        let expected_g_expense_3: f32 = 254.66;
        let expected_income_6: f32 = 0.0;
        let expected_g_income_3: f32 = 1109.42;
        let expected_n_income_5: f32 = 1873.93;

        assert_eq!(expected_date_0, csv_fields.date[0]);
        assert_eq!(expected_g_expense_3, csv_fields.gross_expense[3]);
        assert_eq!(expected_income_6, csv_fields.income[6]);
        assert_eq!(expected_g_income_3, csv_fields.gross_income[3]);
        assert_eq!(expected_n_income_5, csv_fields.net_income[5]);
    }

    #[test]
    fn test_csv_lines() {
        let mut csv_lines = utils::instantiate_csv_lines(None);
        let _csv_fields = CsvLines::compile_csv(&mut csv_lines).unwrap();

        let expected_csv_line_2: StringRecord = StringRecord::from(vec![
            "2024-06-18",
            " $65.52: Groceries",
            " $130.59",
            " $0.00",
            " $1109.42",
            " $978.83",
        ]);

        assert_eq!(expected_csv_line_2, csv_lines.lines[2]);
    }
}
