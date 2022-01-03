pub trait FormStyle {
    fn fields(&self) -> Vec<String>;
    fn set(&mut self, field: &str, value: String);
    fn get(&mut self, field: &str) -> Option<String>;
}
