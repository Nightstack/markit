pub mod cli_save;

pub trait SaveInput {
    fn get_description(&self) -> String;
    fn get_executable(&self) -> bool;
    fn get_content(&self) -> String;
    fn get_tags(&self) -> Vec<String>;
}
