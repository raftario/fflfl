use std::env;
use std::fs;
use std::fs::File;
use std::io::{Read, Write};
use std::process;

fn main() {
    let args: Vec<String> = env::args().collect();
    let args_len = args.len();
    if args_len < 4 {
        eprintln!("Error: Missing arguments. Usage: 'fflfl <INPUT> <OUTPUT> <FILTER>...'.");
        process::exit(1);
    }

    let input_path = args[1].clone();
    let output_path = args[2].clone();
    if fs::metadata(&input_path).is_err() {
        eprintln!("Error: Input file doesn't exists.");
        process::exit(1);
    } else if fs::metadata(&output_path).is_ok() {
        eprintln!("Error: Output file already exists.");
        process::exit(1);
    }

    let filters = {
        let mut f = Vec::new();
        for i in 3..args_len {
            f.push(args[i].clone());
        }
        f
    };

    let mut input = File::open(input_path).unwrap_or_else(|_| {
        eprintln!("Error: Can't open input file.");
        process::exit(1);
    });
    let mut output = File::create(output_path).unwrap_or_else(|_| {
        eprintln!("Error: Can't create output file.");
        process::exit(1);
    });

    let mut unfiltered_contents = String::new();
    input
        .read_to_string(&mut unfiltered_contents)
        .unwrap_or_else(|_| {
            eprintln!("Error: Can't read from input file.");
            process::exit(1);
        });
    let unfiltered_contents: Vec<String> = unfiltered_contents
        .split('\n')
        .map(|l| l.to_owned())
        .collect();

    for l in unfiltered_contents {
        for f in &filters {
            if l.starts_with(f) {
                write!(output, "{}\n", &l).unwrap_or_else(|_| {
                    eprintln!("Error: Can't write to output file.");
                    process::exit(1);
                });
                eprint!("+");
                break;
            }
        }
    }
    eprint!("\n");
    output.flush().unwrap_or_else(|_| {
        eprintln!("Error: Can't flush output file.");
        process::exit(1);
    });
}
