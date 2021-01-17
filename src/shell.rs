pub mod actions;
pub mod kinds;
pub mod utilities;

pub use kinds::Bash;
pub use kinds::Zsh;
pub use utilities::detect;

pub trait Shell {
    fn edit(&self, path: &str) -> String;
    fn init(&self, binary: &str) -> String;
    fn set_var(&self, var: &str, value: &str) -> String;
    fn set_alias(&self, var: &str, value: &str) -> String;
    fn set_function(&self, var: &str, value: &str) -> String;
    fn unset_var(&self, var: &str) -> String;
    fn unset_alias(&self, var: &str) -> String;
    fn unset_function(&self, var: &str) -> String;
}
