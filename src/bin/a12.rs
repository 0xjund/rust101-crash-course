// Topic: Implementing functionality with the impl keyword
//
// Requirements:
// * Print the characteristics of a shipping box
// * Must include dimensions, weight, and color
//
// Notes:
// * Use a struct to encapsulate the box characteristics
// * Use an enum for the box color
// * Implement functionality on the box struct to create a new box
// * Implement functionality on the box struct to print the characteristics

// Think about composability and modularity when you write code and move away from hard coding
// everything!
// This makes it easier to change things later on 
enum Color {
    Brown, 
    Red
}

impl Color {
    fn print(&self) {
        match self {
            Color::Brown => println!("brown"),
            Color::Red => println!("red"), 
        }
    } 
}

struct Dimensions {
    width: f64, 
    height: f64, 
    depth: f64, 
}

impl Dimensions {
    fn print(&self) {
       println!("width: {:?}", self.width); 
       println!("width: {:?}", self.height);
       println!("depth: {:?}", self.depth);
    }
}

struct ShippingBox {
    color:Color, 
    weight: f64, 
    dimensions: Dimensions
}

impl ShippingBox {
    fn new(weight: f64, color:Color, dimensions: Dimensions) -> Self {
        Self {weight, color, dimensions}
    }
    fn print(&self) {
        self.color.print(); 
        self.dimensions.print(); 
        println!("weight: {:?}", self.weight);  
    }
}

fn main() {
    let small_dimensions = Dimensions {
        width: 1.0, 
        height: 2.20, 
        depth: 3.0, 
    };
    let small_box = ShippingBox::new(5.0, Color::Red, small_dimensions); 
    small_box.print(); 
}

// My attempt
//#[derive(Debug)]
//enum BoxColors {
//    Red,
//    Yellow,
//    Orange,
//    Green,
//}

//struct Box {
//    color: BoxColors,
//    dimensions: f64,
//    weight: f64, 
//}

//impl Box {
//    fn new_box() -> Self {
//       Self {
//        color: BoxColors::Red,     
//        dimensions: 21.0,
//        weight: 45.0, 
//        } 
//    }
//    fn print_box(&self) { 
//        println!("Color: {:?}, Dimensions: {}, Weight {}", 
//        self.color, self.dimensions, self.weight); 
//    }
//}

//fn main() {
//    let box1 = Box {
//        color: BoxColors::Orange, 
//        dimensions: 62.1,
//        weight: 42.0
//:weight    }; 
//    box1.print_box(); 
//}
