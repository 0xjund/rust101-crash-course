// Topic: Looping using the loop statement
//
// Program requirements:
//  * Display "1" through "4" in the terminal
//
// Notes:
//  * Use a mutable integer variable
//  * Use a loop statement
//  * Print the variable within the loop statement
//  * Use break to exit the loop

fn main() {
    let mut count_down = 4;
    loop {
        println!("{:?}", count_down);
        count_down = count_down - 1;
        if count_down == 0 {
            break;
        }
    }
    println!("Job's done!");
}
