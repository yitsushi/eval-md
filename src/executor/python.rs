use std::process::{Command, Stdio};
use std::io::Write;

use super::Executor;

pub struct Python;

impl Executor for Python {
    fn exec(&self, script: Vec<String>, argv: Vec<String>) -> std::process::Child {
        let mut args = vec!["-".to_string()];
        args.extend(argv);

        let mut prog = Command::new("python3")
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
            "#!/usr/bin/env python3".into(),
            "".into(),
        ];

        header.extend(script);

        header.join("\n")
    }
}

impl Python {
    pub fn new() -> Python {
        Python{}
    }
}
