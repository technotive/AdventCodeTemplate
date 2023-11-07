pub const ONE: &str = "0123456789";
pub const TWO: &str = "9876543210";

pub fn one(input: &String) -> String {
    input.lines().fold(String::new(), |s, l| format!("{s}{l}"))
}
pub fn two(input: &String) -> String {
    input.lines().rev().fold(String::new(), |s, l| format!("{s}{l}"))
}
