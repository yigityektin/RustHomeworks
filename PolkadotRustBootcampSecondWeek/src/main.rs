use std::io;

enum Operation {
    Add(f64, f64),
    Subtract(f64, f64),
    Multiply(f64, f64),
    Divide(f64, f64),
}

fn calculate(operation: Operation) -> f64 {
    match operation {
        Operation::Add(a, b) => a + b,
        Operation::Subtract(a, b) => a - b,
        Operation::Multiply(a, b) => a * b,
        Operation::Divide(a, b) => a / b, 
    }
}
fn main() {
    println!("Enter the first number: ");
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("err");
    let first_num: f64 = input.trim().parse().expect("err");

    println!("Enter the operation (+, -, /, *): ");
    io::stdin().read_line(&mut input).expect("err");
    let operation = match input.trim() {
        "+" => Operation::Add,
        "-" => Operation::Subtract,
        "*" => Operation::Multiply,
        "/" => Operation::Divide,
        _ => {
            return;
        }
    };

    println!("Enter the second number: ");
    input.clear();
    io::stdin().read_line(&mut input).expect("err");
    let second_num: f64 = input.trim().parse().expect("err");

    let operation_x = operation(first_num, second_num);
    let res = calculate(operation_x);
    println!("Result: {}", res);
}


