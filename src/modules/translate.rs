use std::io::Error;
use super::dictionary;

pub fn translate(option: char, input: &str) -> Result<String, Error> {
    let mut output = String::new();

    match option {
        // Text to Morse
        '1' => {
            let message = input.to_uppercase();
            for c in message.chars() {
                match dictionary::DICTIONARY.get_by_left(&c) {
                    Some(letter) => {
                        output.push_str(*letter);
                        output.push(' ');
                    },
                    None => {println!("\nError! Letter `{}` not found!", c);}
                }
            }
        },
        // Morse to Text
        _ => {
            let words = input.split(" / ");
            for word in words {
                let letters = word.split(' ');
                for letter in letters {
                    match dictionary::DICTIONARY.get_by_right(letter) {
                        Some(l) => output.push(*l),
                        None => {println!("\nError! Letter `{}` not found!", letter);}
                    }
                }

                output.push(' ');
            }
        }
    }

    Ok(output)
}
