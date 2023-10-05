use std::process::Child;

mod javascript;
mod lua;
mod php;
mod python;
mod ruby;
mod shell;

pub use javascript::JavaScript;
pub use lua::Lua;
pub use php::Php;
pub use python::Python;
pub use ruby::Ruby;
pub use shell::Shell;

use crate::code_container::CodeContainer;

pub trait Executor {
    fn exec(&self, script: CodeContainer, argv: Vec<String>) -> Child;
    fn export(&self, script: CodeContainer) -> String;
    fn binary(&self) -> &'static str;
}

pub fn language_picker(executor: &str) -> Option<Box<dyn Executor>> {
    let (lang, executor) = if let Some((l, e)) = resolve_alias(executor) {
        (l, Some(e))
    } else {
        (executor, None)
    };

    match lang {
        "javascript" => {
            let js = if let Some(executor) = executor {
                JavaScript::new(executor)
            } else {
                JavaScript::default()
            };
            Some(Box::new(js))
        },
        "lua" => Some(Box::new(Lua::new())),
        "python" => Some(Box::new(Python::new())),
        "ruby" => Some(Box::new(Ruby::new())),
        "php" => Some(Box::new(Php::new())),
        "shell" => {
            let sh = if let Some(executor) = executor {
                Shell::new(executor)
            } else {
                Shell::default()
            };
            Some(Box::new(sh))
        },
        _ => None
    }
}

pub fn supported_languages() -> Vec<&'static str> {
    vec![
        "javascript",
        "lua",
        "php",
        "python",
        "ruby",
        "shell",
    ]
}

pub fn aliases() -> Vec<(&'static str, &'static str, &'static str)> {
    vec![
        ("bash", "shell", "bash"),
        ("zsh", "shell", "zsh"),
        ("js", "javascript", "node"),
        ("node", "javascript", "node"),
        ("deno", "javascript", "deno"),
    ]
}

pub fn resolve_alias(name: &str) -> Option<(&'static str, &'static str)> {
    for (alias, lang, executor) in aliases() {
        if alias == name {
            return Some((lang, executor))
        }
    }

    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_resolve_alias() {
        let test_cases: Vec<(&str, Option<(&str, &str)>)> = vec![
            ("bash", Some(("shell", "bash"))),
            ("zsh", Some(("shell", "zsh"))),
            ("deno", Some(("javascript", "deno"))),
            ("something", None),
        ];

        for case in test_cases {
            let result = resolve_alias(case.0);
            assert_eq!(result, case.1);
        }
    }

    #[test]
    fn test_language_picker() {
        let test_cases: Vec<(&str, Option<&str>)> = vec![
            ("bash", Some("bash")),
            ("zsh", Some("zsh")),
            ("shell", Some("zsh")),
            ("deno", Some("deno")),
            ("javascript", Some("node")),
            ("js", Some("node")),
            ("something", None),
        ];

        for case in test_cases {
            let result = language_picker(case.0);
            if case.1.is_none() {
                assert!(result.is_none());
                continue
            }

            assert_eq!(Some(result.unwrap().binary()), case.1);
        }
    }
}
