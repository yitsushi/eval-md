use std::process::{Command, Stdio};
use std::io::Write;

use crate::code_container::CodeContainer;

use super::Executor;

pub struct Lua;

impl Executor for Lua {
    fn exec(&self, script: CodeContainer, argv: Vec<String>) -> std::process::Child {
        let mut prog = Command::new("lua")
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
            "#!/usr/bin/env lua".into(),
            "".into(),
        ];

        header.push(script.lines());

        header.join("\n")
    }

    fn binary(&self) -> &'static str {
        "lua"
    }
}

impl Lua {
    pub fn new() -> Lua {
        Lua{}
    }

    fn args(&self, args: Vec<String>) -> Vec<String> {
        let mut argv = vec!["-".to_string()];
        argv.extend(args);
        argv
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_export() {
        let mut code = CodeContainer::new();
        code.open_new_group();
        code.push("print(\"check\")".into());
        code.close_group();

        let lang = Lua::new();
        let output = lang.export(code);
        let expected_output = "#!/usr/bin/env lua\n\nprint(\"check\")".to_string();
        assert_eq!(output, expected_output);
    }

    #[test]
    fn test_args() {
        let lang = Lua::new();
        let args = lang.args(vec!["--my-flag".into(), "-o".into(), "file".into()]);
        let expected_args = vec!["-", "--my-flag", "-o", "file"];
        assert_eq!(args, expected_args);
    }

    #[test]
    fn test_lua_binary() {
        let lang = Lua::new();
        assert_eq!(lang.binary(), "lua")
    }
}
