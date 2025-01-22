// Topic: Flow control using if..else if..else
//
// Display ">5", "<5", or "=5" based on the value of a variable is > 5, < 5, or == 5, respectively
// * Use an if..else if..else block to determine which message to display
// * Use the println macro to display messages to the terminal

fn main() {
    let var_number = 7;
    if var_number > 5 {
        println!("Greater than five");
    } else if var_number < 5 {
        println!("Less than five");
    } else if var_number == 5 {
        println!("Equal to five");
    }
}
