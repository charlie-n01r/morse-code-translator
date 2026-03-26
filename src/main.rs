use std::io::{stdin, stdout, Write};
mod dictionary;

fn main() {
    print!("Insert text to translate: ");
    stdout().flush().unwrap();
    let mut input = String::new();

    match stdin().read_line(&mut input) {
        Ok(_) => {
            let message = &input.trim().to_uppercase();
            for c in message.chars() {
                if let Some(value) = dictionary::DICTIONARY.get_by_left(&c) {
                    print!("{} ", value);
                }
            }
            println!("")
        },
        Err(err) => println!("{}", err)
    }
}
