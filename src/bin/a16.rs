// Topic: Option
//
// Requirements:
// TODO * Print out the details of a student's locker assignment
//  * Lockers use numbers and are optional for students
//
// Notes:
//  * Use a struct containing the student's name and locker assignment
// * The locker assignment should use an Option<i32>

//  * Use a struct containing the student's name and locker assignment
// * The locker assignment should use an Option<i32>

struct LockerName {
    locker_assignment: Option<i32>,
    student_name: String,
}

fn main() {
    let student1 = LockerName {
        locker_assignment: Some(22),
        student_name: "John".to_owned(),
    };
    println!("student: {:?}", student1.student_name);

    match student1.locker_assignment {
        Some(num) => println!("locker number: {:?}", num),
        _ => println!("No locker recorded"),
    }
    let student2 = LockerName {
        locker_assignment: Some(24),
        student_name: "Ash".to_owned(),
    };
    println!("student: {:?}", student2.student_name);
    match student2.locker_assignment {
        Some(num2) => println!("locker number: {:?}", num2),
        _ => println!("No locker recorded"),
    }
}
