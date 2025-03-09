use std::io;

fn main() {

    println!("Welcome to Rust CLI Calculator by @DismasNdadila");
    println!("Enter 1 for Simple Calculator");
    println!("Enter 2 for Advanced Calculator");

    let mut choice = String::new();
    io::stdin()
        .read_line(&mut choice)
        .expect("Failed to read line");

    let choice: i32 = choice.trim().parse().expect("Please type a number!");

    match choice {
        1 => simple_calculator(),
        2 => advanced_calculator(),
        _ => println!("Invalid choice"),
    }
 
}

fn simple_calculator(){
   
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

fn advanced_calculator(){

 println!("Rust CLI Calculator");
    println!("Enter numbers followed by an operator (e.g., '+', '-', '*', '/').");
    println!("Type '=' to calculate the result.");
    println!("Type 'exit' to quit.");

    loop {
        let mut numbers: Vec<f64> = Vec::new(); 
        let mut operator = String::new();

        println!("Enter numbers (type '=' to calculate):");

  
        loop {
            let mut input = String::new();
            io::stdin().read_line(&mut input).expect("Failed to read input");

            let trimmed = input.trim();

            if trimmed == "=" { break; } 
            if trimmed == "exit" { return; } 

            match trimmed.parse::<f64>() {
                Ok(num) => numbers.push(num),
                Err(_) => {
                    println!("Invalid number! Please enter a valid number.");
                }
            }
        }

        println!("Enter operator (+, -, *, /):");
        io::stdin().read_line(&mut operator).expect("Failed to read input");
        let operator = operator.trim();

        let mut result = numbers[0]; 

        for &num in &numbers[1..] {
            result = match operator {
                "+" => result + num,
                "-" => result - num,
                "*" => result * num,
                "/" => {
                    if num != 0.0 {
                        result / num
                    } else {
                        println!("Error: Cannot divide by zero.");
                        return;
                    }
                }
                _ => {
                    println!("Invalid operator.");
                    return;
                }
            };
        }

        println!("Result: {}", result);
    }
}