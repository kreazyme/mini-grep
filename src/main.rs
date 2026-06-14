use std::env;

use mini_grep::{read_file, search_case_insensitive};

fn main() {
    let args: Vec<String> = env::args().collect();
    print!("{:?}", args);
    if args.len() < 2 {
        println!("Please provide a file path as an argument.");
        return;
    }
    let mut file_content = read_file(args[1].clone());

    if !args[2].is_empty() {
        file_content = search_case_insensitive(file_content, args[2].clone()).join("\n");
    }
    println!("{}", file_content);
}
