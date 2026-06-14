use std::env;

use mini_grep::read_file;

fn main() {
    let args: Vec<String> = env::args().collect();
    print!("{:?}", args);
    if args.len() < 2 {
        println!("Please provide a file path as an argument.");
        return;
    }
    let file_content = read_file(args[1].clone());
    println!("{}", file_content);
}
