use std::io::{stdin, stdout, Write};


pub fn str_input(provided_message: &str, options: Vec<&str>) -> String {
    println!("{}", provided_message);

    let mut ret_str: String = String::new();
    let mut breaker: bool = false;
    loop {

        let _=stdout().flush();
        stdin().read_line(&mut ret_str).expect("Did not enter a correct string");

        for i in options.clone().into_iter() {
            if i == ret_str {
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