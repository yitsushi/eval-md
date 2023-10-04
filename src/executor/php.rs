use std::process::{Command, Stdio};
use std::io::Write;

use super::Executor;

pub struct Php;

impl Executor for Php {
    fn exec(&self, script: Vec<String>, argv: Vec<String>) -> std::process::Child {
        let mut prog = Command::new("php")
            .args(self.args(argv))
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
            "#!/usr/bin/env php".into(),
            "".into(),
        ];

        header.extend(script);

        header.join("\n")
    }

    fn binary(&self) -> &'static str {
        "php"
    }
}

impl Php {
    pub fn new() -> Php {
        Php{}
    }

    fn args(&self, args: Vec<String>) -> Vec<String> {
        let mut argv = vec!["--".to_string()];
        argv.extend(args);
        argv
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_export() {
        let lang = Php::new();
        let output = lang.export(vec!["<?php echo \"check\" ?>".into()]);
        let expected_output = "#!/usr/bin/env php\n\n<?php echo \"check\" ?>".to_string();
        assert_eq!(output, expected_output);
    }

    #[test]
    fn test_args() {
        let lang = Php::new();
        let args = lang.args(vec!["--my-flag".into(), "-o".into(), "file".into()]);
        let expected_args = vec!["--", "--my-flag", "-o", "file"];
        assert_eq!(args, expected_args);
    }

    #[test]
    fn test_php_binary() {
        let lang = Php::new();
        assert_eq!(lang.binary(), "php")
    }
}
