// Topic: Browsing standard library documentation
//
// Requirements:
// TODO * Print a string in lowercase and uppercase
//
// Notes:
// * Utilize standard library functionality to
//   transform the string to lowercase and uppercase
// * Use 'rustup doc' in a terminal to open the standard library docs
// * Navigate to the API documentation section
// * Search for functionality to transform a string (or str)
//   to uppercase and lowercase
//   * Try searching for: to_uppercase, to_lowercase

fn main() {
    let new_string = "This is a string";
    println!("uppercase {:?}", new_string.to_uppercase());
    println!("lowercase {:?}", new_string.to_lowercase());
}
