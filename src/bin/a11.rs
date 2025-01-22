// Topic: Ownership
//
// Requirements:
// * Print out the quantity and id number of a grocery item
//
// Notes:
// * Use a struct for the grocery item
// * Use two i32 fields for the quantity and id number
// * Create a function to display the quantity, with the struct as a parameter
// * Create a function to display the id number, with the struct as a parameter

struct Item {
    quantity: i32, 
    id_number: i32, 
}

fn display_quant(item: &Item) {
    println!("quantity = {:?}", item.quantity);
}

fn display_id(item: &Item) {
    println!("quantity = {:?}", item.id_number); 
}

fn main() {
    let item = Item {
        quantity: 12, 
        id_number: 24, 
    };
    display_quant(&item); 
    display_id(&item)
}
