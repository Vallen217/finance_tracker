use super::Pathing;
use super::*;

impl Pathing {
    pub fn generate_file_path(
        date: &utils::Date,
        create_file: bool,
    ) -> Result<Pathing, Box<dyn Error>> {
        let user_dir = match dirs::home_dir() {
            Some(dir) => dir,
            None => panic!("Error: unable to determine $HOME directory"),
        };
        let parent_dir = format!(
            "{}/Documents/Health/Macronutritional_Intake",
            user_dir.to_str().unwrap()
        );

        let pathing = Pathing {
            year_path: format!("{parent_dir}/{}", date.year),
            month_path: format!("{parent_dir}/{}/{}", date.year, date.month),
            day_path: format!("{parent_dir}/{}/{}/{}.txt", date.year, date.month, date.day),
        };

        if create_file {
            Pathing::create_file(&pathing)?;
        }
        Ok(pathing)
    }

    pub fn create_file(&self) -> Result<(), Box<dyn Error>> {
        fs::create_dir_all(&self.month_path)?;

        let _ = fs::OpenOptions::new()
            .write(true)
            .create_new(true)
            .open(&self.day_path)?;

        Ok(())
    }
}

pub fn file_exists(path: &String) -> bool {
    path::Path::new(&path).exists()
}

pub fn user_path() -> Result<String, Box<dyn Error>> {
    let dir_path = match dirs::home_dir() {
        Some(dir) => dir,
        None => panic!("Error: unable to determine $HOME directory"),
    };

    Ok(dir_path.to_str().unwrap().to_string())
}

// NOTE: It won't be necessary to get the 'day' segment.
pub fn user_input_pathing(
    parent_directory: String,
    date_type: &str,
) -> Result<String, Box<dyn Error>> {
    let parent_dir = fs::read_dir(&parent_directory)?;

    println!("\nEnter a {} from:", date_type);
    for path in parent_dir {
        println!("{}", path?.path().display());
    }

    let mut path = String::new();
    io::stdin().read_line(&mut path)?;

    let mut formatted_path = format!("{}/{}", parent_directory, &path[0..path.len() - 1]);

    if date_type.contains("day") || date_type.contains("pd file") {
        if path.contains(".txt") {
            formatted_path = format!("{}/{}", parent_directory, &path[0..path.len() - 1]);
        } else {
            formatted_path = format!("{}/{}.txt", parent_directory, &path[0..path.len() - 1]);
        }
    }

    // for user to quit prematurely
    if formatted_path.contains("q") {
        process::exit(0);
    }

    if !file_exists(&formatted_path) {
        println!("\nError: Invalid selection");
        return user_input_pathing(parent_directory, date_type);
    }

    Ok(formatted_path)
}

#[cfg(test)]
mod unit_tests {
    use super::*;

    fn instantiate_test_paths() -> Pathing {
        let month_path = match fs::canonicalize("./test_files/good_files") {
            Ok(path) => path,
            Err(err) => {
                dbg!(err);
                panic!();
            }
        };
        let month_path = month_path.to_str().unwrap().to_string();

        let day_path = format!("{}/file_1.csv", month_path);
        let test_pathing = Pathing {
            year_path: "none".to_string(),
            month_path,
            day_path,
        };

        test_pathing
    }

    #[test]
    fn test_create_file() {
        let test_pathing = instantiate_test_paths();

        let _ = Pathing::create_file(&test_pathing);
    }

    #[test]
    fn test_file_exits() {
        let test_pathing = instantiate_test_paths();

        assert!(file_exists(&test_pathing.day_path));
    }
}
