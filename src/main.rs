use std::{io::{self, BufRead, BufReader, Lines}, fs::File, path::Path};

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

    let lines = match read_lines(arguments.file.clone()) {
        Ok(it) => it,
        _ => {
            println!(" -- File not found: {}", arguments.file);
            return
        },
    };

    let (name, executor) = extract_language(arguments.language.as_str());
    let content: Vec<String> = extract_content(name, lines);
    let lang = executor::language_picker(executor);

    if arguments.debug {
        println!(" -- Target Language: {}", name);
        println!(" -- Target Executor: {}", executor);
    }

    if arguments.export {
        println!("{}", lang.unwrap().export(content));

        return
    }

    if lang.is_none() {
        let (supported, alias_list) = help_available();
        println!(" -- unknown language: {}", arguments.language);
        println!("available languages:\n{}\n", supported);
        println!("aliases:\n{}", alias_list);

        return
    }


    let prog = lang.unwrap().exec(content, arguments.args);
    prog.wait_with_output().expect("Failed to read stdout");
}

fn help_available() -> (String, String) {
    let supported = executor::supported_languages()
        .iter()
        .map(|l| format!(" - {}", l))
        .collect::<Vec<String>>()
        .join("\n");

    let alias_list = executor::aliases()
        .iter()
        .map(|(a, n, e)| format!(" - {:<15} => {}::{}", a, n, e))
        .collect::<Vec<String>>()
        .join("\n");
    (supported, alias_list)
}

fn extract_content(name: &str, lines: Lines<BufReader<File>>) -> Vec<String> {
    let mut open = false;
    lines.into_iter().fold(Vec::<String>::new(), |mut c, line| {
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
    })
}

fn extract_language(lang: &str) -> (&str, &str) {
    if !lang.contains(':') {
        return (lang, lang);
    }

    let parts = lang.split(':').collect::<Vec<&str>>();

    (parts.first().unwrap().to_owned(), parts.get(1).unwrap().to_owned())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_extract_language() {
        let test_cases: Vec<(&str, &str, &str)> = vec![
            ("python", "python", "python"),
            ("py:python", "py", "python"),
            ("py:", "py", ""),
            ("", "", ""),
        ];

        for case in test_cases {
            let (name, exec) = extract_language(case.0);
            assert_eq!(name, case.1);
            assert_eq!(exec, case.2);
        }
    }
}
