use super::*;

impl Date {
    pub fn current_date() -> Date {
        let chrono_date = format!("{}", Local::now().date_naive());
        let mut date_segments = vec![];

        for val in chrono_date.split("-") {
            let val: i16 = match val.parse() {
                Ok(data) => data,
                Err(error) => {
                    dbg!(error);
                    panic!("Error: compiling current date '{}'", val);
                }
            };
            date_segments.push(val);
        }

        let date = Date {
            year: date_segments[0],
            month: date_segments[1],
            day: date_segments[2],
        };

        date
    }
}

pub fn csv_date_form() -> String {
    let date = Date::current_date();

    let month: String;
    if date.month < 10 {
        month = format!("0{}", date.month);
    } else {
        month = format!("{}", date.month);
    }

    let day: String;
    if date.day < 10 {
        day = format!("0{}", date.day);
    } else {
        day = format!("{}", date.day);
    }

    let csv_date = format!("{}-{}-{}", date.year, month, day);
    csv_date
}

pub fn instantiate_csv_fields() -> CsvFields {
    let expense = instantiate_expense_fields();
    let csv_fields = CsvFields {
        date: vec![],
        expense,
        gross_expense: vec![],
        income: vec![],
        gross_income: vec![],
        net_income: vec![],
    };

    csv_fields
}

pub fn instantiate_expense_fields() -> ExpenseFields {
    let expense_fields = ExpenseFields {
        expense: vec![],
        commodity: vec![],
    };

    expense_fields
}

pub fn instantiate_csv_lines(path: Option<String>) -> CsvLines {
    let file_path = match path {
        Some(file_path) => file_path,
        None => {
            String::from("/home/vallen/Workspace/finance_tracker/test_files/good_files/file_1.csv")
        }
    };
    let csv_lines = CsvLines {
        file_path,
        lines: vec![],
    };

    csv_lines
}

// NOTE: 'map' is cloned during the implementations of this function in a 'for loop'.
// As such this is a relativly costly function to implement.
pub fn get_value_key(value: &f32, mut map: HashMap<&String, f32>) -> String {
    let mut value_key = String::new();
    for key in map.clone().keys() {
        if map.remove(key).unwrap() == *value {
            value_key = key.to_string();
        }
    }

    value_key
}

#[allow(dead_code)]
pub fn user_test_path() -> String {
    let dir_path = match fs::canonicalize("..") {
        Ok(path) => path,
        Err(err) => {
            dbg!(err);
            panic!();
        }
    };

    format!("{}/macro_counter/", dir_path.to_str().unwrap().to_string())
}

#[cfg(test)]
mod unit_tests {
    use super::*;

    #[test]
    fn test_date_len() {
        let csv_date = utils::csv_date_form();
        assert_eq!(csv_date.len(), 10);
    }
}
