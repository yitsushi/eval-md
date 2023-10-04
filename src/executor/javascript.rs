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
        let mut prog = Command::new(self.target_str())
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
            format!("#!/usr/bin/env {}", self.target_str()),
            "".into(),
        ];

        header.extend(script);

        header.join("\n")
    }

    fn binary(&self) -> &'static str {
        self.target_str()
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

    fn args(&self, args: Vec<String>) -> Vec<String> {
        let mut argv = match self {
            JavaScript::Deno => vec!["run".to_string(), "-".to_string()],
            JavaScript::Node => vec!["-".to_string()],
        };
        argv.extend(args);
        argv
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_export() {
        let lang = JavaScript::default();
        let output = lang.export(vec!["console.log(\"check\")".into()]);
        let expected_output = "#!/usr/bin/env node\n\nconsole.log(\"check\")".to_string();
        assert_eq!(output, expected_output);
    }

    #[test]
    fn test_default_args() {
        let lang = JavaScript::default();
        let args = lang.args(vec!["--my-flag".into(), "-o".into(), "file".into()]);
        let expected_args = vec!["-", "--my-flag", "-o", "file"];
        assert_eq!(args, expected_args);
    }

    #[test]
    fn test_node_export() {
        let lang = JavaScript::new("node");
        let output = lang.export(vec!["console.log(\"check\")".into()]);
        let expected_output = "#!/usr/bin/env node\n\nconsole.log(\"check\")".to_string();
        assert_eq!(output, expected_output);
    }

    #[test]
    fn test_node_args() {
        let lang = JavaScript::new("node");
        let args = lang.args(vec!["--my-flag".into(), "-o".into(), "file".into()]);
        let expected_args = vec!["-", "--my-flag", "-o", "file"];
        assert_eq!(args, expected_args);
    }

    #[test]
    fn test_deno_export() {
        let lang = JavaScript::new("deno");
        let output = lang.export(vec!["console.log(\"check\")".into()]);
        let expected_output = "#!/usr/bin/env deno\n\nconsole.log(\"check\")".to_string();
        assert_eq!(output, expected_output);
    }

    #[test]
    fn test_deno_args() {
        let lang = JavaScript::new("deno");
        let args = lang.args(vec!["--my-flag".into(), "-o".into(), "file".into()]);
        let expected_args = vec!["run", "-", "--my-flag", "-o", "file"];
        assert_eq!(args, expected_args);
    }

    #[test]
    fn test_deno_binary() {
        let lang = JavaScript::new("deno");
        assert_eq!(lang.binary(), "deno")
    }

    #[test]
    fn test_node_binary() {
        let lang = JavaScript::new("node");
        assert_eq!(lang.binary(), "node")
    }
}
