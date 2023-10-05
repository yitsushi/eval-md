use std::process::{Command, Stdio};
use std::io::Write;

use crate::code_container::CodeContainer;

use super::Executor;

#[derive(Default)]
pub enum Shell {
    Bash,
    #[default]
    Zsh,
}

impl Executor for Shell {
    fn exec(&self, script: CodeContainer, argv: Vec<String>) -> std::process::Child {
        let mut prog = Command::new(self.target_str())
            .args(self.args(argv))
            .stdin(Stdio::piped())
            .spawn()
            .unwrap();

        let mut stdin = prog.stdin.take().expect("Failed to open stdin");
        std::thread::spawn(move || {
            stdin.write_all(script.lines().as_bytes()).expect("Failed to write to stdin");
        });

        prog
    }

    fn export(&self, script: CodeContainer) -> String {
        let mut header: Vec<String> = vec![
            format!("#!/usr/bin/env {}", self.target_str()),
            "".into(),
        ];

        header.push(script.lines());

        header.join("\n")
    }

    fn binary(&self) -> &'static str {
        self.target_str()
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

    fn target_str(&self) -> &'static str {
        match self {
            Shell::Bash => "bash",
            Shell::Zsh => "zsh",
        }
    }

    fn args(&self, args: Vec<String>) -> Vec<String> {
        let mut argv = vec!["/dev/stdin".to_string()];
        argv.extend(args);
        argv
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_export() {
        let mut code = CodeContainer::new();
        code.open_new_group();
        code.push("echo \"check\"".into());
        code.close_group();

        let lang = Shell::default();
        let output = lang.export(code);
        let expected_output = "#!/usr/bin/env zsh\n\necho \"check\"".to_string();
        assert_eq!(output, expected_output);
    }

    #[test]
    fn test_default_args() {
        let lang = Shell::default();
        let args = lang.args(vec!["--my-flag".into(), "-o".into(), "file".into()]);
        let expected_args = vec!["/dev/stdin", "--my-flag", "-o", "file"];
        assert_eq!(args, expected_args);
    }

    #[test]
    fn test_zsh_export() {
        let mut code = CodeContainer::new();
        code.open_new_group();
        code.push("echo \"check\"".into());
        code.close_group();

        let lang = Shell::new("zsh");
        let output = lang.export(code);
        let expected_output = "#!/usr/bin/env zsh\n\necho \"check\"".to_string();
        assert_eq!(output, expected_output);
    }

    #[test]
    fn test_zsh_args() {
        let lang = Shell::new("zsh");
        let args = lang.args(vec!["--my-flag".into(), "-o".into(), "file".into()]);
        let expected_args = vec!["/dev/stdin", "--my-flag", "-o", "file"];
        assert_eq!(args, expected_args);
    }

    #[test]
    fn test_bash_export() {
        let mut code = CodeContainer::new();
        code.open_new_group();
        code.push("echo \"check\"".into());
        code.close_group();

        let lang = Shell::new("bash");
        let output = lang.export(code);
        let expected_output = "#!/usr/bin/env bash\n\necho \"check\"".to_string();
        assert_eq!(output, expected_output);
    }

    #[test]
    fn test_bash_args() {
        let lang = Shell::new("bash");
        let args = lang.args(vec!["--my-flag".into(), "-o".into(), "file".into()]);
        let expected_args = vec!["/dev/stdin", "--my-flag", "-o", "file"];
        assert_eq!(args, expected_args);
    }

    #[test]
    fn test_bash_binary() {
        let lang = Shell::new("bash");
        assert_eq!(lang.binary(), "bash")
    }

    #[test]
    fn test_zsh_binary() {
        let lang = Shell::new("zsh");
        assert_eq!(lang.binary(), "zsh")
    }
}
