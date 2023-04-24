use std::process::{Command, Stdio};
use std::io::Write;

use super::Executor;

#[derive(Default)]
pub enum JavaScript {
    Deno,
    #[default]
    Node,
}

impl Executor for JavaScript {
    fn exec(&self, script: Vec<String>, argv: Vec<String>) -> std::process::Child {
        let mut args = match self {
            JavaScript::Deno => vec!["run".to_string(), "-".to_string()],
            JavaScript::Node => vec!["-".to_string()],
        };
        args.extend(argv);

        let mut prog = Command::new(self.target_str())
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

    fn export(&self, script: Vec<String>) -> String {
        let mut header: Vec<String> = vec![
            format!("#!/usr/bin/env {}", self.target_str()),
            "".into(),
        ];

        header.extend(script);

        header.join("\n")
    }
}

impl JavaScript {
    pub fn new(runner: &str) -> JavaScript {
        match runner {
            "node" => JavaScript::Node,
            "deno" => JavaScript::Deno,
            _ => JavaScript::default(),
        }
    }

    fn target_str(&self) -> &'static str {
        match self {
            JavaScript::Deno => "deno",
            JavaScript::Node => "node",
        }
    }
}
