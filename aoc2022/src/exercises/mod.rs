pub mod day1;

pub trait Exercise {
    fn get_name(&self) -> Option<String>;
    fn execute(&self) -> Result<String, String>;
}