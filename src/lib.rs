use std::fs;

pub fn read_file(path: String) -> String {
    let file_content = fs::read_to_string(path);
    return match file_content {
        Ok(text) => text,
        Err(err) => {
            println!("{}", err);
            return String::from("");
        }
    };
}
