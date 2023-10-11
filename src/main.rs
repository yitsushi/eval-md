use std::{io::{self, BufRead, BufReader, Lines, Write}, fs::File, path::Path};

mod executor;
mod code_block_options;
mod code_container;

use clap::Parser;
use code_block_options::{CodeBlockOption, find_group_name};
use code_container::CodeContainer;

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

    /// Group name.
    #[arg(short, long)]
    group: Option<String>,

    /// Export the scirpt and skip execution.
    /// Export accepts any string value as target language.
    #[arg(short, long)]
    export: bool,

    /// Debug mode.
    #[arg(short, long)]
    debug: bool,

    /// Pick mode will ask each code block to be included in the final script
    /// or not.
    #[arg(short, long)]
    pick: bool,
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
    let content: CodeContainer = extract_content(name, lines, ExtractOptions {
        group: arguments.group,
        pick: arguments.pick,
    });
    let lang = executor::language_picker(executor);

    if arguments.pick {
        // Add an extra empty line to separate "pick" answers.
        eprintln!()
    }

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

#[derive(Default, Debug)]
struct ExtractOptions {
    group: Option<String>,
    pick: bool,
}

fn extract_content(name: &str, lines: Lines<BufReader<File>>, opts: ExtractOptions) -> CodeContainer {
    let pattern = if name == "all" {
        "```".to_string()
    } else {
        format!("```{}", name)
    };
    lines.into_iter().fold(CodeContainer::new(), |mut c, line| {
        if let Ok(line) = line {
            if line.starts_with(&pattern) {
                if opts.group.is_none() {
                    c.open_new_group();
                    return c
                }
                let group = find_group_name(CodeBlockOption::parse_options(&line));
                if group == opts.group.clone().unwrap_or_default() {
                    c.open_new_group();

                    return c
                }
            } else if line == "```" {
                if let Some(block) = c.open_lines() {
                    if !block.is_empty() {
                        if opts.pick && !ask_yes_no(block) {
                            c.discard();
                            return c
                        }

                        c.close_group();
                        return c
                    }
                }

                c.discard()
            } else if c.is_open() {
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

    let mut parts = lang.splitn(2, ':');

    (parts.next().unwrap(), parts.next().unwrap())
}

fn ask_yes_no(block: String) -> bool {
    eprintln!();
    eprintln!("---");
    eprintln!("{}", block);
    eprintln!("---");

    loop {
        let mut buffer = String::new();

        eprint!(" --> Do you want to add this block? (yes/no) ");
        io::stderr().flush().unwrap();

        if io::stdin().read_line(&mut buffer).is_ok() {
            if buffer.trim() == "yes" {
                return true
            }
            if buffer.trim() == "no" {
                return false
            }
        };
    }
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
            ("php", "php", "php"),
            ("", "", ""),
        ];

        for case in test_cases {
            let (name, exec) = extract_language(case.0);
            assert_eq!(name, case.1);
            assert_eq!(exec, case.2);
        }
    }
}
