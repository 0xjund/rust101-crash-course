// Topic: Working with expressions
//
// Requirements:
// TODO * Print "its big" if a variable is > 100
// TODO * Print "its small" if a variable is <= 100
//
// Notes:
//  * Use a boolean variable set to the result of
//   an if..else expression to store whether the value
//   is > 100 or <= 100
// TODO * Use a function to print the messages
// TODO * Use a match expression to determine which message
//   to print

fn print_num(gt_100: bool) {
    match gt_100 {
        true => println!("Greater than 100"),
        false => println!("Less than 100"), 
    }; 

}


fn main() {
    let num = 120; 
    let num_check  = num > 100;
    print_num(num_check); 
   }

