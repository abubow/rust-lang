fn print_menu() {
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
    let mut result = 0;
    loop {
        let mut choice = String::new();
        let mut a = String::new();
        let mut b = String::new();
        print_menu();
        println!("Enter your choice: ");
        std::io::stdin()
            .read_line(&mut choice)
            .expect("Failed to read line");
        let choice: i32 = match choice.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Please type a valid number!");
                continue;
            }
        };
        if choice == 5 {
            println!("Quitting");
            return;
        }
        println!("Enter first number: ");
        std::io::stdin()
            .read_line(&mut a)
            .expect("Failed to read line");
        let a: i32 = match a.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Please type a valid number!");
                continue;
            }
        };
        println!("Enter second number: ");
        std::io::stdin()
            .read_line(&mut b)
            .expect("Failed to read line");
        let b: i32 = match b.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Please type a valid number!");
                continue;
            }
        };
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
