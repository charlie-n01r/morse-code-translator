use clap::{Parser, ArgGroup};
use std::path::PathBuf;
use std::fs;

use crate::modules::translate::translate;
mod modules;

#[derive(Parser, Debug)]
#[command(
    version,
    about,
    long_about = None,
    group(
        ArgGroup::new("mode")
            .args(["to_text", "to_morse"]) // Make either to_text or to_morse mandatory
            .required(true)
            .multiple(false)
    )
)]
struct Cli {
    #[arg(short='t', long="to-text")]
    to_text: bool,

    #[arg(short='m', long="to-morse")]
    to_morse: bool,

    input: Option<String>,

    #[arg(short, long, requires="mode")]
    file: Option<PathBuf>,

    #[arg(short, long, requires="mode")]
    output: Option<PathBuf>,
}


fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Cli::parse();

    // If --file is provided, then it takes the input from the file contents
    let input_text = if let Some(file_path) = args.file {
        fs::read_to_string(file_path)?
    // Otherwise it takes the provided string argument as input
    } else if let Some(text) = args.input {
        text
    } else {
        eprintln!("Error: must provide input text or --file");
        std::process::exit(1);
    };

    // If the flag is --to-morse is provided, send the input to the translator with the flag 1
    let output_text = if args.to_morse {
        translate('1', &input_text)?
    } else if args.to_text {
    // Otherwise send the input to the translator with flag 2
        translate('2', &input_text)?
    } else {
        unreachable!("Clap guarantees one direction is set")
    };

    // If the flag --output was provided, write output to file path
    if let Some(out_path) = args.output {
        std::fs::write(out_path, &output_text)?;
    } else {
    // Otherwise print results to screen
        println!("{}", output_text);
    }

    Ok(())
}
