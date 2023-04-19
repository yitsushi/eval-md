use std::process::Child;

mod python;
mod shell;
mod ruby;

pub use python::Python;
pub use shell::Shell;
pub use ruby::Ruby;

pub trait Executor {
    fn exec(&self, script: Vec<String>, argv: Vec<String>) -> Child;
}
