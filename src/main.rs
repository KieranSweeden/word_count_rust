use std::{fs, io, path};

fn main() {
    intro();

    loop {
        let file_path_string = prompt_user_for_file_path();
        println!("You've provided the path of {}", file_path_string);

        let file_path = path::Path::new(&file_path_string);
        let file_path_is_valid = validate_file_path(&file_path);

        if !file_path_is_valid {
            println!("{} is not a valid file path.", &file_path.display());
            continue;
        }

        let file_contents = get_file_contents(&file_path);

        println!("{}", &file_contents);
    }
}

fn get_file_contents(file_path: &path::Path) -> String {
    let file_contents = fs::read_to_string(file_path)
        .expect("Something went wrong whilst attempting to read the file");

    file_contents
}

fn validate_file_path(file_path: &path::Path) -> bool {
    file_path.is_file()
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
