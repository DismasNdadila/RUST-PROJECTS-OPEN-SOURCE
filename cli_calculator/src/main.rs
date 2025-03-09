use std::io;

fn main() {

println!("Enter the first number: ");

let mut number1 = String::new();

io::stdin().read_line(&mut number1).expect( "Failed to read line");

println!("You entered: {} as the number of your choice", number1.trim() );


}
