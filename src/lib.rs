use std::fs;

pub fn run(config: Config) {
    let filedata = search(config.query, read_file(config.file_name));
    print!("result: {}", filedata.len());
    for line in filedata {
        println!("{}", line);
    }
}

pub struct Config {
    pub file_name: String,
    pub is_sensitive: bool,
    pub query: String,
}

impl Config {
    pub fn new(args: Vec<String>) -> Result<Config, &'static str> {
        let mut data = args.iter();
        data.next();
        let file_name = match data.next() {
            Some(val) => val,
            None => return Err("Please provide a file path as an argument."),
        };

        let is_sensitive = match data.next() {
            Some(val) => val == "true",
            None => return Err("Please specify if the search is sensitive."),
        };

        let query = match data.next() {
            Some(val) => val,
            None => return Err("Please provide a query string."),
        };

        Ok(Config {
            file_name: file_name.clone(),
            is_sensitive,
            query: query.clone(),
        })
    }
}

fn read_file(path: String) -> String {
    print!("Reading file: {}", path);
    let content = fs::read_to_string(path);
    return content.unwrap();
}

fn search(keyword: String, file_content: String) -> Vec<String> {
    file_content
        .lines()
        .filter(|line: &&str| line.contains(&keyword))
        .map(String::from)
        .collect()
}
