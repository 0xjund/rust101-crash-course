// Topic: Data management using tuples
//
// Requirements:
//  * Print whether the y-value of a cartesian coordinate is
//   greater than 5, less than 5, or equal to 5
//
// Notes:
//  * Use a function that returns a tuple
//  * Destructure the return value into two variables(x and y)
//   * Use an if..else if..else block to determine what to print
// My solution, compiles returns correct answer but seems a little long winded 
fn coord(x: i32, y: i32) -> (i32, i32){
     (x,y)  
    // Could change to just (x,y), had "return"
} 

fn main() {
// Updated to direct destructing 
//  let coords = coord(5, 19);
//  let (x,y) = (coords.0, coords.1); 
    let (x,y) = coord(5,19);
    if y < 5 {
    println!("Less than five");
  } else if y > 5 {
        println!("Greater than five"); 
   } else {
       println!("Equal to five"); 
   }

}


// ChatGPT offered corrections- implicit returns, simplfies the destructuring
//
//fn coord(x: i32, y: i32) -> (i32, i32) {
//    (x, y) // Implicit return
//}

//fn main() {
//    let (x, y) = coord(5, 19); // Direct destructuring

//    if y < 5 {
//        println!("Less than five");
//    } else if y > 5 {
//        println!("Greater than five");
//    } else {
//        println!("Equal to five"); // Simplified final branch
//    }
//}

//
