use std::io;

fn main() {
    println!("Enter the first number: ");

    let mut number1 = String::new();

    io::stdin()
        .read_line(&mut number1)
        .expect("Failed to read first number");

    let number1: f64 = number1.trim().parse().expect("Please type a number!");

    println!("Enter the second number: ");
    let mut number2 = String::new();
    io::stdin()
        .read_line(&mut number2)
        .expect("Failed to read second number");
    let number2: f64 = number2.trim().parse().expect("Please type a number!");

    println!("Enter the operation you want to perform (+, -, *, / ) : ");
    let mut operation = String::new();
    io::stdin()
        .read_line(&mut operation)
        .expect("Failed to read Operation");
    let operation = operation.trim();

    let result = match operation {
        "+" => number1 + number2,
        "-" => number1 - number2,
        "*" => number1 * number2,
        "/" => {
            if number2 == 0.0 {
                println!("Error: Division by zero");
                return;
            } else {
                number1 / number2
            }
        }

        _ => panic!("Invalid operation"),
    };

    println!("You wanted to perform {} {} {} = {}", number1, operation, number2, result);
}
