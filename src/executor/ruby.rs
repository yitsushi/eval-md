use std::process::{Command, Stdio};
use std::io::Write;

use super::Executor;

pub struct Ruby;

impl Executor for Ruby {
    fn exec(&self, script: Vec<String>, argv: Vec<String>) -> std::process::Child {
        let mut args = vec!["-".to_string()];
        args.extend(argv);

        let mut prog = Command::new("ruby")
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

impl Ruby {
    pub fn new() -> Ruby {
        Ruby{}
    }
}

