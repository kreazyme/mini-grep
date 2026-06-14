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

pub fn search_case_insensitive(content: String, keyword: String) -> Vec<String> {
    let keyword = keyword.to_lowercase();
    return content
        .lines()
        .filter(|line| line.to_lowercase().contains(&keyword))
        .map(String::from)
        .collect();
}
