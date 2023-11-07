pub fn one(input: &String) -> String {
    input.lines().fold(String::new(), |s, l| format!("{s}{l}"))
}
pub fn two(input: &String) -> String {
    input.lines().rev().fold(String::new(), |s, l| format!("{s}{l}"))
}
