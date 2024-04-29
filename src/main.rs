mod tools;
mod structs;
mod fileman;

use std::{fs::{remove_file, File}, io::{stdin, stdout, Write}, process::exit};

use fileman::fileman::{file_saver, saver};
use tools::str_input;

use crate::{fileman::fileman::opener, structs::ListItem, tools::{list_printer}};


fn main() {
    let mut user_input = String::new();
    let save_file: File;

    user_input = str_input("Would you like to load a list, or create a new one? (l/n)", vec!["l", "n"]);

    if user_input == "n" {
        println!("What is the directory of the file you would like to use?");

        stdout().flush().expect("flush will not fail.");
        user_input.clear();
        stdin().read_line(&mut user_input).expect("Must be a string, incorrect file will be handled later.");

        save_file = opener(&user_input.trim());
    }
    else {
        println!("What is the directory of the file you would like to use?");

        stdout().flush().expect("flush will not fail.");
        user_input.clear();
        stdin().read_line(&mut user_input).expect("Must be a string, incorrect file will be handled later.");

        save_file = opener(&user_input.trim());
    }
    let mut list: Vec<ListItem> = vec![];
    let file_dir: String = String::from(&user_input);
    loop {
        stdout().flush().expect("flush will not fail");
        user_input = str_input("What would you like to do to the list?\n\nCreate a new item (n)\nRemove an item (r)\nDelete the list (will exit after deleting) (d)\nList all current items (l)\nSave and exit (se)\nExit without saving (e)\n\n", vec!["n","r","d","l","se","e"]);
        match user_input.as_str().trim() {
            "n"=> {
                println!("What would you like the addition to say?\n");

                stdout().flush().expect("flush will not fail");
                user_input.clear();
                stdin().read_line(&mut user_input).expect("Input must be a string");

                list.push(ListItem {
                        item_id: list.len().try_into().expect("must be u128"),
                        contents: user_input,
                    });

                println!("Item added. Current list status shown below:\n\n");
                list_printer(&list);
                println!();
            },
            "r"=> {
                println!("Which item would you like to remove? Showing list below;\n");
                list_printer(&list);

                let mut breaker: bool = false;
                let mut idx: u32 = 0;
                loop {

                    stdin().read_line(&mut user_input).expect("Error will be handled shortly.");

                    idx = match user_input.parse() {
                        Ok(n) => {
                            breaker = true;
                            n
                        },
                        Err(e) => {
                            eprintln!("{}", e);
                            0
                        }
                    };

                    if breaker {
                        break;
                    }

                    println!("Invalid value entered, please try again.\n");
                    user_input.clear();
                }
            },
            "d"=> {
                let res = remove_file(file_dir.clone());
                match res {
                    Ok(_e) => {
                        println!("File deleted successfully, exiting.");
                        exit(0)
                    },
                    Err(e) => {
                        println!("File failed to delete, exiting via panic.");
                        panic!("{}", e);
                    },
                };
            },
            "l"=> {
                list_printer(&list);
                println!();
            },
            "se"=> {
                println!("Saving and exiting.");
                file_saver(&save_file, list);
                break;
            },
            "e" => {
                println!("Exiting.");
                break;
            },
            _=> {
                panic!("if you got this output you did something veeeery wrong. terminating.")
            },
        };
    };

}