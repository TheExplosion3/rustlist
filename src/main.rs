mod tools;
mod structs;

use std::io::{stdin, stdout, Write};

use tools::str_input;

use crate::structs::ListItem;


fn main() {
    let mut user_input = String::new();

    user_input = str_input("Would you like to load a list, or create a new one? (s/n)", vec!["s", "n"]);
    if user_input == "n" {
        println!("What is the directory of the file you would like to use?");
        stdin().read_line(&mut user_input);

    }
    else {
        println!("What is the directory of the file you would like to use?");
        stdin().read_line(&mut user_input);
    }

}
