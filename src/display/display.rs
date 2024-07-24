use super::*;

pub fn display_file(path: String) {
    match fs::read_to_string(&path) {
        Ok(file) => println!("\n{}", file),
        Err(_) => {
            println!("Error: unable to read '{}'", path);
            crate::main();
        }
    }
}
