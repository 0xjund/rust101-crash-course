// Topic: Strings
//
// Requirements:
// * Print out the name and favorite colors of people aged 10 and under
//
// Notes:
//  * Use a struct for a persons age, name, and favorite color
//  * The color and name should be stored as a String
//  * Create and store at least 3 people in a vector
//  * Iterate through the vector using a for..in loop
//  * Use an if expression to determine which person's info should be printed
//  * The name and colors should be printed using a function

struct PersonalInfo {
    age: i32,
    name: String,
    fav_color: String,
}

fn print_info(personalinfo: &PersonalInfo) {
    println!(
        "Name: {}, Favorite Color: {}",
        personalinfo.name, personalinfo.fav_color
    );
}

fn main() {
    let person_store = vec![
        PersonalInfo {
            age: 24,
            name: "James".to_string(),
            fav_color: "Red".to_string(),
        },
        PersonalInfo {
            age: 16,
            name: "Liz".to_string(),
            fav_color: "Yellow".to_string(),
        },
        PersonalInfo {
            age: 8,
            name: "Joe".to_string(),
            fav_color: "Purple".to_string(),
        },
    ];

    for person in &person_store {
        if person.age <= 10 {
            println!(
                "Name: {}, Age: {}, Favorite Color: {}",
                person.name, person.age, person.fav_color
            );
        }
    }

    for person in &person_store {
        print_info(person);
    }
}
