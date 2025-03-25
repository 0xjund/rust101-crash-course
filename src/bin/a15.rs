// Topic: Advanced match
//
// Requirements:
// TODO * Print out a list of tickets and their information for an event
//  * Tickets can be Backstage, Vip, and Standard
//  * Backstage and Vip tickets include the ticket holder's name
//  * All tickets include the price
//
// Notes:
//  * Use an enum for the tickets with data associated with each variant
//  * Create one of each ticket and place into a vector
//  * Use a match expression while iterating the vector to print the ticket info

enum TicketType {
    Backstage(f64, String),
    Standard(f64),
    Vip(f64, String),
}

fn main() {
    let ticket_store = vec![
        TicketType::Backstage(150.00, "John".to_owned()),
        TicketType::Standard(50.00),
        TicketType::Vip(75.00, "Jim".to_owned()),
    ];
    for ticket_type in ticket_store {
        match ticket_type {
            TicketType::Backstage(price, holder) => {
                println!("Holder: {:?}, price {:?}", holder, price)
            }
            TicketType::Standard(price) => println!("price {:?}", price),
            TicketType::Vip(price, holder) => println!("Holder: {:?}, price {:?}", holder, price),
        }
    }
}
