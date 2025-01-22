// Topic: Working with an enum
//
// Program requirements:
// TODO * Prints the name of a color to the terminal
//
// Notes:
// * Use an enum with color names as variants
//  * Use a function to print the color name
//  * The function must use the enum as a parameter
// * Use a match expression to determine which color name to print

enum Colors {
    Red,
    Orange,
    Blue,
    Green,
    Purple,
}

fn display_color(print_color: Colors) {
    match print_color {
        Colors::Red => {
            println!("Red");
        }
        Colors::Orange => {
            println!("Orange");
        }
        Colors::Blue => {
            println!("Blue");
        }
        Colors::Green => {
            println!("Green");
        }
        Colors::Purple => {
            println!("Purple");
        }
    }
}

fn main() {
    display_color(Colors::Orange);
}
