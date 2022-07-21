// Topic: Option
//
// Requirements:
// * Print out the details of a student's locker assignment
// * Lockers use numbers and are optional for students
//
// Notes:
// * Use a struct containing the student's name and locker assignment
// * The locker assignment should use an Option<i32>

// * Use a struct containing the student's name and locker assignment
// * The locker assignment should use an Option<i32>

struct Student {
  name: String,
  locker: Option<i32>
}

fn main() {
  let students = vec![
    Student { name: "Joe".to_owned(), locker: Some(1) },
    Student { name: "Jane".to_owned(), locker: None },
    Student { name: "Pete".to_owned(), locker: Some(2) },
  ];

  for student in students {
    match student {
      Student { name, locker } => match locker {
        Some(l) => println!("{:?} is assigned locked #{:?}", name, l),
        None => println!("{:?} is not assigned a locker!", name)
      }
    }
  };
}