use std::process::Child;

mod javascript;
mod lua;
mod python;
mod ruby;
mod shell;

pub use javascript::JavaScript;
pub use lua::Lua;
pub use python::Python;
pub use ruby::Ruby;
pub use shell::Shell;

pub trait Executor {
    fn exec(&self, script: Vec<String>, argv: Vec<String>) -> Child;
    fn export(&self, script: Vec<String>) -> String;
}

pub fn language_picker(executor: &str) -> Option<Box<dyn Executor>> {
    let (lang, executor) = if let Some((l, e)) = resolve_alias(executor) {
        (l, Some(e))
    } else {
        (executor, None)
    };

    match lang {
        "javascript" => {
            if let Some(executor) = executor {
            Some(Box::new(JavaScript::new(executor)))
            } else {
                Some(Box::new(JavaScript::default()))
            }
        },
        "lua" => Some(Box::new(Lua::new())),
        "python" => Some(Box::new(Python::new())),
        "ruby" => Some(Box::new(Ruby::new())),
        "shell" => {
            if let Some(executor) = executor {
                Some(Box::new(Shell::new(executor)))
            } else {
                Some(Box::new(Shell::default()))
            }
        },
        _ => None
    }
}

pub fn supported_languages() -> Vec<&'static str> {
    vec![
        "javascript",
        "lua",
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
