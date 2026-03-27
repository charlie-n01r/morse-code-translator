use std::io::{stdin, stdout, Write, Error};
mod dictionary;

fn translate(option: &str) -> Result<(), Error> {
    let mut input = String::new();
    loop {
        // Clear the string before each loop and read input
        input.clear();
        print!("Insert text to translate [send `|` to quit]: ");
        stdout().flush().unwrap();
        match stdin().read_line(&mut input) {
            Ok(_) => {
                let trimmed = input.trim();
                // Quit
                if trimmed == "|" {
                    return Ok(())
                }

                match option {
                    // Text to Morse
                    "1" => {
                        let message = trimmed.to_uppercase();
                        for c in message.chars() {
                            match dictionary::DICTIONARY.get_by_left(&c) {
                                Some(letter) => print!("{} ", letter),
                                None => {
                                    println!("\nError! Letter not found!");
                                    continue;
                                }
                            }
                        }
                    },
                    // Morse to Text
                    _ => {
                        let words = trimmed.split(" / ");
                        for word in words {
                            let letters = word.split(' ');
                            for letter in letters {
                                match dictionary::DICTIONARY.get_by_right(letter) {
                                    Some(l) => print!("{}", l),
                                    None => {
                                        println!("\nError! Letter not found!");
                                        continue;
                                    }
                                }
                            }

                            print!(" ");
                        }
                    }
                }
            },
            // Return io Error
            Err(err) => return Err(err)
        }

        println!("");
    }
}

fn menu() -> Result<(), Error> {
    let mut option = String::new();
    print!("---Menu---\n[1] Plain text to Morse Code\n[2] Morse Code to Plain text\n\n> ");
    stdout().flush().unwrap();

    match stdin().read_line(&mut option) {
        Ok(_) => {
            let trimmed = option.trim();
            match trimmed  {
                // Only valid options are 1 (text to morse) or 2 (morse to text)
                "1" | "2" => translate(trimmed),
                _ => {
                    println!("Incorrect option. Try again.");
                    Ok(())
                }
            }
        },
        Err(err) => Err(err)
    }
}

fn main() {
    match menu() {
        Ok(_) => (),
        Err(err) => println!("{}", err)
    }
}
