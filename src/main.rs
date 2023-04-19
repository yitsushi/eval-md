use std::{io::{self, BufRead}, fs::File, path::Path};

mod executor;

use clap::Parser;
use executor::Executor;

#[derive(Debug, Parser)]
struct Args {
    language: String,
    file: String,
    args: Vec<String>,
    #[arg(short, long)]
    debug: bool,
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn main() {
    let arguments = Args::parse();

    if arguments.debug {
        println!(" -- Target Language: {}", arguments.language);
        println!(" -- Source file: {}", arguments.file);
        println!(" -- Arguments: {:?}", arguments.args);
    }

    if let Ok(lines) = read_lines(arguments.file) {
        let mut open = false;
        let content: Vec<String> = lines.into_iter().fold(Vec::<String>::new(), |mut c, line| {
            if let Ok(line) = line {
                if line == format!("```{}", arguments.language) {
                    open = true;
                } else if line == "```" {
                    open = false;
                } else if open {
                    c.push(line);
                }
            }

            c
        });

        let lang: Box<dyn Executor> = match arguments.language.as_str() {
            "python" => Box::new(executor::Python::new()),
            "bash" => Box::new(executor::Shell::new("bash")),
            "zsh" => Box::new(executor::Shell::new("zsh")),
            "ruby" => Box::new(executor::Ruby::new()),
            _ => {
                println!(" -- unknown language: {}", arguments.language);
                println!("    available languages:");
                println!("     - python");
                println!("     - bash");
                println!("     - zsh");
                println!("     - ruby");
                return
            },
        };

        let prog = lang.exec(content, arguments.args);

        prog.wait_with_output().expect("Failed to read stdout");
    }
}
