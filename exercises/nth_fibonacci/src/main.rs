use std::io;

fn main() {
    let number = read_number();
    let ordinal = get_ordinal(number);
    let result = calculate_nth_fibonacci(number);
    println!("The {number}{ordinal} Fibonacci number is {result}");
}

fn read_number() -> u32 {
    loop {
        println!("Please input a number between 1 and 100, inclusive:");
        
        let mut value = String::new();

        match io::stdin().read_line(&mut value) {
            Ok(_) => (),
            Err(_) => {
                println!("Failed to read line");
                continue;
            }
        };

        let value = match value.trim().parse() {
            Ok(num) if num < 1 || num > 100 => {
                println!("Number falls outside the expected range, try again");
                continue;
            },
            Ok(num) => num,
            Err(_) => {
                println!("Invalid number");
                continue;
            }
        };

        break value;
    }
}

fn get_ordinal(number: u32) -> String {
    let ordinal = match number % 10 {
        1 => "st",
        2 => "nd",
        3 => "rd",
        _ => "th",
    }.to_string();
    ordinal
}

fn calculate_nth_fibonacci(number: u32) -> u32 {
    if number < 3 {
        return  1;
    }
    
    let mut nth;
    let mut n1th = 1;
    let mut n2th = 2;
    if number > 3 {
        for _ in 3..number {
            nth = n1th;
            n1th = n2th;
            n2th = nth + n1th;
        }
    }
    n2th
}