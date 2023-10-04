use std::str::FromStr;

const CB_OPTION_GROUP: &str = "group";

#[derive(Debug, Eq, PartialEq)]
pub struct CodeBlockOption {
    pub key: String,
    pub value: String,
}

impl CodeBlockOption {
    pub fn is_group(&self) -> bool {
        self.key == CB_OPTION_GROUP
    }

    pub fn parse_options(line: &str) -> Vec<CodeBlockOption> {
        if !line.contains('#') {
            return vec![]
        }
        let parts = line.split('#');
        if let Some(options) = parts.last() {
            println!(" -- {} -> {:?}", line, options);
            let options = options.split(' ')
                          .filter_map(|x| CodeBlockOption::from_str(x).ok())
                          .collect::<Vec<CodeBlockOption>>();
            options
        } else {
            vec![]
        }
    }
}

impl std::str::FromStr for CodeBlockOption {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.is_empty() {
            return Err("empty string found for options list".into())
        }
        if !s.contains('=') {
            return Ok(CodeBlockOption{key: s.into(), value: "".into()})
        }
        let mut parts = s.splitn(2, '=');
        Ok(CodeBlockOption{
            key: parts.next().unwrap().into(),
            value: parts.next().unwrap().into()
        })
    }
}

pub fn find_group_name(options: Vec::<CodeBlockOption>) -> String {
    let res = options
        .into_iter()
        .filter(|x| x.is_group())
        .map(|x| x.value)
        .collect::<Vec<String>>();

    res.first().cloned().unwrap_or_default()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn new_cbo<S: Into<String>>(key: S, value: S) -> CodeBlockOption {
        CodeBlockOption { key: key.into(), value: value.into() }
    }

    #[test]
    fn test_code_block_option_from_str() {
        let test_cases: Vec<(&str, Option<CodeBlockOption>)> = vec![
            ("group=a", Some(CodeBlockOption { key: "group".into(), value: "a".into() })),
            ("empty=", Some(CodeBlockOption { key: "empty".into(), value: "".into() })),
            ("single", Some(CodeBlockOption { key: "single".into(), value: "".into() })),
            ("more=than=one=eq", Some(CodeBlockOption { key: "more".into(), value: "than=one=eq".into() })),
            ("", None),
        ];

        for case in test_cases {
            let result = CodeBlockOption::from_str(case.0);
            assert_eq!(result.ok(), case.1);
        }
    }

    #[test]
    fn test_parse_options() {
        let test_cases: Vec<(&str, Vec<CodeBlockOption>)> = vec![
            ("```bash", vec![]),
            ("```bash #", vec![]),
            ("```bash #group=a", vec![new_cbo("group", "a")]),
            ("```bash #group=a version=3", vec![new_cbo("group", "a"), new_cbo("version", "3")]),
            ("```bash # group=a", vec![new_cbo("group", "a")]),
        ];

        for case in test_cases {
            let result = CodeBlockOption::parse_options(case.0);
            assert_eq!(result, case.1);
        }
    }
}
