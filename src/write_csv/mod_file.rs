use super::*;

pub fn mod_file(mut fields: CsvFields, path: String) {
    let mut oper = String::new();
    match io::stdin().read_line(&mut oper) {
        Ok(oper) => oper,
        Err(_) => {
            println!("Error: unable to read operation '{}'", &oper);
            return mod_file(fields, path.clone());
        }
    };

    let re = Regex::new(r"rlq?[0-9]*").unwrap();
    if re.is_match(&oper) {
        remove_lines(&mut fields, oper.clone());
        let _ = CsvFields::write_csv(&mut fields, path.clone());
        return crate::main();
    }

    if oper.trim() == "q" {
        display_file(path.clone());
        return crate::main();
    } else {
        let _ = CsvFields::compile_input(&mut fields, path);
    }
}

fn remove_lines(fields: &mut CsvFields, oper: String) {
    loop {
        let iter: i8 = if oper.contains("q") {
            // remove 1 file line if the number of lines to remove is not specified.
            if &oper.trim().len() < &4 {
                1
            } else {
                match oper.clone().trim()[3..].parse() {
                    Ok(data) => data,
                    Err(error) => {
                        dbg!(error);
                        panic!("Error: parsing operation '{}'", oper);
                    }
                }
            }
        } else {
            // remove 1 file line if the number of lines to remove is not specified.
            if &oper.trim().len() < &3 {
                1
            } else {
                match oper.clone().trim()[2..].parse() {
                    Ok(data) => data,
                    Err(error) => {
                        dbg!(error);
                        panic!("Error: parsing operation '{}'", oper);
                    }
                }
            }
        };

        for _ in 0..iter {
            let _ = delete_last_line(fields);
        }

        return;
    }
}

fn delete_last_line(fields: &mut CsvFields) -> Result<(), Box<dyn Error>> {
    fields.date.pop();
    fields.expense.expense.pop();
    fields.expense.commodity.pop();
    fields.gross_expense.pop();
    fields.income.pop();
    fields.gross_income.pop();
    fields.net_income.pop();

    Ok(())
}
