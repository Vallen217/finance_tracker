use super::*;
use crate::Date;

impl CsvLines {
    pub fn compile_re_exp(&mut self) -> Result<CsvFields, Box<dyn Error>> {
        let user_home_dir = match dirs::home_dir() {
            Some(dir) => dir,
            None => panic!("Error: unable to determine $HOME directory"),
        };
        let re_exp_dir = format!(
            "{}/Documents/Finance/Records/recurrent_expenses.csv",
            user_home_dir.to_str().unwrap()
        );

        let csv_reader = csv::ReaderBuilder::new()
            .has_headers(true)
            .from_path(re_exp_dir);

        for result in csv_reader?.records() {
            let record = result?;
            let re_exp_due = match self.re_exp_due(record.clone()) {
                Ok(ok) => ok,
                Err(err) => {
                    dbg!(
                        "Error: compiling recurrent expenses\n
                        {:#?}\nAborting potentially predefined expenses.",
                        &err
                    );
                    return Err(err);
                }
            };
            if re_exp_due {
                self.lines.push(record);
            };
        }

        let csv_fields = self.parse_csv_line(utils::instantiate_csv_fields());
        Ok(csv_fields)
    }

    // Checks if a recurring expense entry is due
    fn re_exp_due(&self, re_exp_line: StringRecord) -> Result<bool, Box<dyn Error>> {
        let re_exp_day = match Self::get_re_exp_day(re_exp_line) {
            Ok(day) => day,
            Err(err) => {
                dbg!("Error: retrieving recurrent expense day\n{:#?}", &err);
                return Err(err);
            }
        };
        let last_entry_day = match self.get_last_entry_day() {
            Ok(day) => day,
            Err(err) => {
                dbg!("Error: retrieving last csv date segment: day\n{:#?}", &err);
                return Err(err);
            }
        };
        let current_day = Date::current_date().day;

        return Ok((re_exp_day > last_entry_day) && (re_exp_day <= current_day));
    }

    // get the 'day' from a the last main csv line
    fn get_last_entry_day(&self) -> Result<i16, Box<dyn Error>> {
        let mut last_entry_day = String::new();
        for val in self.lines.last().unwrap() {
            let val = val.trim().to_string();
            if val.len() < 10 {
                last_entry_day = Self::format_re_exp_date(val);
            } else {
                last_entry_day = val;
            }
            break;
        }

        let day: i16 = match last_entry_day[8..].parse() {
            Ok(day) => day,
            Err(err) => return Err(Box::new(err)),
        };

        Ok(day)
    }

    // get the 'day' from a passed re_exp csv line
    fn get_re_exp_day(re_exp_line: StringRecord) -> Result<i16, Box<dyn Error>> {
        let mut re_exp_day = String::new();

        for val in &re_exp_line {
            re_exp_day = if val.len() > 2 {
                val.trim()[8..].to_string()
            } else {
                val.trim().to_string()
            };
            break;
        }

        let day: i16 = match re_exp_day.parse() {
            Ok(day) => day,
            Err(err) => return Err(Box::new(err)),
        };

        Ok(day)
    }

    // The recurring expense csv file exclusively uses days by default,
    // not complete date formats.
    pub fn format_re_exp_date(mut re_exp_day: String) -> String {
        let date = Date::current_date();
        // Ensuring there are 2 digits in the 'month' & 'day' segments (e.g. "04" not "4")
        let temp_month = format!("{}", date.month);
        let month = if temp_month.len() != 2 {
            format!("0{}", temp_month)
        } else {
            temp_month
        };

        re_exp_day = if re_exp_day.len() != 2 {
            format!("0{}", re_exp_day)
        } else {
            re_exp_day
        };

        let full_date = format!("{}-{}-{}", date.year, month, re_exp_day);
        return full_date;
    }
}
