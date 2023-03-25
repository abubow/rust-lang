fn printMenu() {
    println!("1. Add");
    println!("2. Subtract");
    println!("3. Multiply");
    println!("4. Divide");
    println!("5. Quit");
}

fn add(a: i32, b: i32) -> i32 {
    a + b
}

fn subtract(a: i32, b: i32) -> i32 {
    a - b
}

fn multiply(a: i32, b: i32) -> i32 {
    a * b
}

fn divide(a: i32, b: i32) -> i32 {
    a / b
}

fn main() {
    let mut choice = 0;
    let mut a = 0;
    let mut b = 0;
    let mut result = 0;
    while choice != 5 {
        printMenu();
        println!("Enter your choice: ");
        std::io::stdin().read_line(&mut choice).expect("Failed to read line");
        if choice == 5 {
            println!("Quitting");
            return;
        }
        println!("Enter first number: ");
        std::io::stdin().read_line(&mut a);
        println!("Enter second number: ");
        std::io::stdin().read_line(&mut b);
        match choice {
            1 => result = add(a, b),
            2 => result = subtract(a, b),
            3 => result = multiply(a, b),
            4 => result = divide(a, b),
            _ => println!("Invalid choice"),
        }
        println!("Result: {}", result);
    }
}
