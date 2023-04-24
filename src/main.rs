use std::{io::{self, BufRead}, fs::File, path::Path};

mod executor;

use clap::Parser;

#[derive(Debug, Parser)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Language to extract.
    /// Executor can be defined with ':', for example:
    /// js:node or py:python.
    language: String,
    /// Source file.
    file: String,
    /// Arguments to the script.
    args: Vec<String>,

    /// Export the scirpt and skip execution.
    /// Export accepts any string value as target language.
    #[arg(short, long)]
    export: bool,

    /// Debug mode.
    #[arg(short, long)]
    debug: bool,
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn main() {
    let arguments: Args = Args::parse();

    if arguments.debug {
        println!(" -- Target Language: {}", arguments.language);
        println!(" -- Source file: {}", arguments.file);
        println!(" -- Arguments: {:?}", arguments.args);
    }

    if let Ok(lines) = read_lines(arguments.file) {
        let (name, executor) = if arguments.language.contains(':') {
            let parts = arguments.language.split(':').collect::<Vec<&str>>();

            (parts.first().unwrap().to_owned(), parts.get(1).unwrap().to_owned())
        } else {
            (arguments.language.as_str(), arguments.language.as_str())
        };

        if arguments.debug {
            println!(" -- Target Language: {}", name);
            println!(" -- Target Executor: {}", executor);
        }

        let mut open = false;
        let content: Vec<String> = lines.into_iter().fold(Vec::<String>::new(), |mut c, line| {
            if let Ok(line) = line {
                if line == format!("```{}", name) {
                    open = true;
                } else if line == "```" {
                    open = false;
                } else if open {
                    c.push(line);
                }
            }

            c
        });

        let lang = if let Some(lang) = executor::language_picker(executor) {
            lang
        } else {
            if arguments.export {
                println!("{}", content.join("\n"));

                return
            }


            println!(" -- unknown language: {0}\n\
                     available languages:\n\
                     {1}\n\
                     \n\
                     aliases:\n\
                     {2}\n\
                     ",
                     arguments.language,
                     executor::supported_languages()
                                .iter()
                                .map(|l| format!(" - {}", l))
                                .collect::<Vec<String>>()
                                .join("\n"),
                     executor::aliases()
                                .iter()
                                .map(|(a, n, e)| format!(" - {:<15} => {}::{}", a, n, e))
                                .collect::<Vec<String>>()
                                .join("\n")
            );

            return
        };

        if arguments.export {
            println!("{}", lang.export(content));

            return
        }


        let prog = lang.exec(content, arguments.args);

        prog.wait_with_output().expect("Failed to read stdout");
    }
}
