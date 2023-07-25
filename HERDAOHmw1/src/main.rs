use std::io;

struct Student {
    name: String,
    age: u32,
    grade: u8,
}

enum GradeLevel {
    Freshman,
    Sophomore,
    Junior,
    Senior,
}

fn add_student(students: &mut Vec<Student>) {
    let mut name = String::new();
    io::stdin().read_line(&mut name).expect("Please enter a real name");

    let mut age = String::new();
    io::stdin().read_line(&mut age).expect("Please enter a valid age");
    let age: u32 = age.trim().parse().expect("Please enter a valid age");

    let mut grade = String::new();
    io::stdin().read_line(&mut grade).expect("Please enter a valid grade");
    let grade: u8 = grade.trim().parse().expect("Please enter a valid grade");

    let new_std = Student {
        name: name.trim().to_string(),
        age,
        grade,
    };

    students.push(new_std);
}

fn display_students(students: &Vec<Student>) {
    for student in students {
        println!("Name: {}, Age: {}, Grade: {}", student.name, student.age, student.grade);
    }
}

fn get_average_grade(students: &Vec<Student>) -> f64 {
    let total: u32 = students.iter().map(|student| u32::from(student.grade)).sum();
    let num_of_std = students.len() as u32;
    f64::from(total) / f64::from(num_of_std)
}

fn get_highest_grade(students: &Vec<Student>) -> Option<&Student> {
    students.iter().max_by_key(|student| student.grade)
}

fn search_student(students: &Vec<Student>, name: &str) {
    let std = students.iter().find(|student| student.name == name);
    match std {
        Some(student) => {
            println!("Name: {}, Age: {}, Grade: {}", student.name, student.age, student.grade);
        }
        None => println!("Not found"),
    }
}

fn main() {
    let mut students: Vec<Student> = Vec::new();

    loop {
        println!("1. Add a new student");
        println!("2. Display all students");
        println!("3. Calculate average grade");
        println!("4. Find student with the highest grade");
        println!("5. Search for a specific student");
        println!("6. Exit");

        let mut choice = String::new();
        io::stdin().read_line(&mut choice).expect("Failed to read line");
        let choice: u8 = match choice.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Invalid choice");
                continue;
            }
        };

        match choice {
            1 => add_student(&mut students),
            2 => display_students(&students),
            3 => {
                let avg = get_average_grade(&students);
                println!("Average grade: {}", avg);
            }
            4 => {
                if let Some(student) = get_highest_grade(&students) {
                    println!("Name: {}, Age: {}, Grade: {}", student.name, student.age, student.grade);
                } else {
                    println!("Not found");
                }
            }
            5 => {
                println!("Enter the name of the student:");
                let mut name = String::new();
                io::stdin().read_line(&mut name).expect("Failed to read line");
                search_student(&students, &name.trim());
            }
            6 => break,
            _ => println!("Invalid choice"),
        }
    }
}
