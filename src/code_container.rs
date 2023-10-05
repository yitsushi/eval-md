#[derive(Debug)]
pub struct CodeContainer {
    blocks: Vec<Vec<String>>,
    open: Option<Vec<String>>,
}

impl CodeContainer {
    pub fn new() -> Self {
        Self{ blocks: vec![], open: None }
    }

    pub fn open_new_group(&mut self) {
        self.open = Some(vec![]);
    }

    pub fn close_group(&mut self) {
        if let Some(block) = self.open.clone() {
            self.blocks.push(block);
            self.open = None;
        };
    }

    pub fn discard(&mut self) {
        self.open = None
    }

    pub fn push(&mut self, line: String) {
        self.open = match self.open.clone() {
            Some(mut block) => {
                block.push(line);
                Some(block)
            },
            None => None,
        };
    }

    pub fn is_open(&self) -> bool {
        self.open.is_some()
    }

    pub fn lines(&self) -> String {
        self.blocks.iter()
            .map(|x| x.join("\n"))
            .collect::<Vec<String>>()
            .join("\n")
    }

    pub fn open_lines(&self) -> Option<String> {
        if let Some(block) = self.open.clone() {
            return Some(block.join("\n"))
        }

        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_open_new_group() {
        let mut code = CodeContainer::new();
        assert!(!code.is_open());
        code.open_new_group();
        assert!(code.is_open());
    }

    #[test]
    fn test_close_group() {
        let mut code = CodeContainer::new();
        assert!(!code.is_open());

        code.open_new_group();
        assert!(code.is_open());
        code.close_group();
        assert!(!code.is_open());
    }

    #[test]
    fn test_close_group_lines() {
        let mut code = CodeContainer::new();
        assert!(!code.is_open());

        {
            code.open_new_group();
            assert!(code.is_open());

            code.push("line 1".into());
            code.push("line 2".into());

            code.close_group();
            assert!(!code.is_open());
        }

        code.push("line 3".into());

        assert_eq!(code.lines(), "line 1\nline 2");
    }

    #[test]
    fn test_discard() {
        let mut code = CodeContainer::new();
        assert!(!code.is_open());

        {
            code.open_new_group();
            assert!(code.is_open());

            code.push("line 1".into());
            code.push("line 2".into());

            code.close_group();
            assert!(!code.is_open());
        }

        {
            code.open_new_group();
            assert!(code.is_open());

            code.push("line 3".into());

            code.discard();
            assert!(!code.is_open());
        }

        {
            code.open_new_group();
            assert!(code.is_open());

            code.push("line 4".into());

            code.close_group();
            assert!(!code.is_open());
        }

        assert_eq!(code.lines(), "line 1\nline 2\nline 4");
    }

    #[test]
    fn test_open_lines() {
        let mut code = CodeContainer::new();
        assert!(!code.is_open());

        {
            code.open_new_group();
            assert!(code.is_open());

            code.push("line 1".into());
            code.push("line 2".into());

            assert_eq!(code.open_lines(), Some("line 1\nline 2".into()));

            code.close_group();
            assert!(!code.is_open());
        }

        code.push("line 3".into());
        assert!(code.open_lines().is_none());
    }

}
