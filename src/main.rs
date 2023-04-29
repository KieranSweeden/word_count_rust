use std::io;

fn main() {
    intro();

    loop {
        let path = prompt_user_for_file_path();
        println!("You've provided the path of {}", path);
    }
}

fn prompt_user_for_file_path() -> String {
    println!("Please provide a path to the target file:");

    let mut input = String::new();

    io::stdin()
        .read_line(&mut input)
        .expect("No input was provided");

    input.trim().to_string()
}

fn intro() {
    println!("=====================");
    println!("Welcome to word count");
    println!("=====================");
}
