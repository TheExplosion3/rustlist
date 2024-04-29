use std::io::{stdin, stdout, Write};

use crate::structs::ListItem;


pub fn str_input(provided_message: &str, options: Vec<&str>) -> String {
    println!("{}", provided_message);

    let mut ret_str: String = String::new();
    let mut breaker: bool = false;
    loop {

        let _=stdout().flush();
        stdin().read_line(&mut ret_str).expect("Did not enter a correct string");

        for i in options.clone().into_iter() {
            if i == ret_str.trim() {
                breaker = true;
                break;
            }
        }

        if breaker {
            break;
        }
        ret_str.clear();
        println!("Input was invalid, please try again.");
    }
    return ret_str;
}

pub fn list_printer(list: &Vec<ListItem>) {
    for i in list.clone().into_iter() {
        println!("{}. {}", i.item_id, i.contents);
        if i.item_id == 0 {
            println!();
            continue;
        }
        if i.item_id - 1 == list.len().try_into().expect("list size has to be positive int") {
            println!();
        }
    }
}