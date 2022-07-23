// Topic: Lifetimes & Functions
//
// Summary:
// Create a program that compares which string is longer (highest length).
//
// Requirements:
// * The comparison must be done using a function named `longest`
// * No data may be copied (cannot use .to_owned() or .to_string())
// * If both strings are the same length, the first one should be returned

fn longest<'a>(str1: &'a str, str2: &'a str) -> &'a str {
    if str1.len() == str2.len() {
        return str1;
    } else if str1.len() > str2.len() {
        return str1;
    } else {
        return str2;
    }
}
fn main() {
    let short = "hello";
    let long = "this is a long message";
    println!("{}", longest(&short, &long))
}
