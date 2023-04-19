use std::process::{Command, Stdio};
use std::io::Write;

use super::Executor;

#[derive(Default)]
pub enum Shell {
    Bash,
    #[default]
    Zsh,
}

impl Executor for Shell {
    fn exec(&self, script: Vec<String>, argv: Vec<String>) -> std::process::Child {
        let mut args = vec!["/dev/stdin".to_string()];
        args.extend(argv);

        let target = match self {
            Self::Bash => "bash",
            Self::Zsh => "zsh",
        };

        let mut prog = Command::new(target)
            .args(args)
            .stdin(Stdio::piped())
            .spawn()
            .unwrap();

        let mut stdin = prog.stdin.take().expect("Failed to open stdin");
        std::thread::spawn(move || {
            stdin.write_all(script.join("\n").as_bytes()).expect("Failed to write to stdin");
        });

        prog
    }
}

impl Shell {
    pub fn new(shell: &str) -> Shell {
        match shell {
            "bash" => Shell::Bash,
            "zsh" => Shell::Zsh,
            _ => Shell::default(),
        }
    }
}
