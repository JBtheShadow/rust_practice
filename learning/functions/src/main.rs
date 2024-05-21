fn main() {
    println!("Hello, world!");

    another_function(5);
    print_labeled_measurements(5, 'h');
    assign_expression();
}

fn another_function(x: i32) {
    println!("The value of x is: {x}");
}

fn print_labeled_measurements(value: i32, unit_label: char) {
    println!("The measurement is: {value}{unit_label}");
}

fn assign_expression() {
    // This groups the statements and expressions and assigns y with the result 4
    let y = {
        let x = three();
        // IMPORTANT: no semicolon on this last line
        // Semicolons turn expressions into statements
        x + 1 
    };

    println!("The value of y is: {y}");

    let x = five();
    println!("The value of x is: {x}");

    let x = plus_one(5);
    println!("The value of x is: {x}");
}

fn three() -> i32 {
    // You can omit the word return if the function ends in an expression
    // Then the expression becomes the return value
    3 
}

fn five() -> i32 {
    5
}

fn plus_one(x: i32) -> i32 {
    x + 1
}