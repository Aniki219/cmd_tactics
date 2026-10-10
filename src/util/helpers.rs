// 2. Define a trait to add a method to slices
pub trait ToVecString {
    fn to_strings(&self) -> Vec<String>;
}

// 3. Implement the trait for any slice of &str
impl ToVecString for [&str] {
    fn to_strings(&self) -> Vec<String> {
        self.iter().map(|&s| s.to_string()).collect()
    }
}