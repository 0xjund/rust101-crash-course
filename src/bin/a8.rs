// Topic: Organizing similar data using structs
//
// Requirements:
// * Print the flavor of a drink and it's fluid ounces
//
// Notes:
// * Use an enum to create different flavors of drinks
// Use a struct to store drink flavor and fluid ounce information
// * Use a function to print out the drink flavor and ounces
// * Use a match expression to print the drink flavor

enum Flavors {
    Choco,
    Strawberry,
    Coffee,
    Vanilla,
}

struct DrinkInfo {
    flavor: Flavors,
    ounce: f64,
}

fn call_info(drink: DrinkInfo) {
    match drink.flavor {
        Flavors::Choco => println!("Chocolate Flavored"),
        Flavors::Strawberry => println!("Strawberry Flavored"),
        Flavors::Coffee => println!("Coffee Flavored"),
        Flavors::Vanilla => println!("Vanilla Flavored"),
    }
    println!("oz: {:?}", drink.ounce);
}

fn main() {
    let drink_choice = DrinkInfo {
        flavor: Flavors::Choco,
        ounce: 20.01,
    };
    call_info(drink_choice);
}
