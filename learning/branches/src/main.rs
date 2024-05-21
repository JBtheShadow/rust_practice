fn main() {
    let number = 7;

    if number < 5 {
        println!("condition was true");
    } else {
        println!("condition was false");
    }

    // Condition must be a bool, other types will cause an error
    if number != 0 {
        println!("number was something other than zero")
    }

    if number % 4 == 0 {
        println!("number is divisible by 4");
    } else if number % 3 == 0 {
        println!("number is divisible by 3");
    } else if number % 2 == 0 {
        println!("number is divisible by 2");
    } else {
        println!("number is not divisible by 4, 3 or 2");
    }

    let mod11 = 3210 % 11;
    let checksum = if mod11 > 9 {0} else if mod11 == 0 {1} else {mod11};

    println!("The value of checksum is: {checksum}");
}
