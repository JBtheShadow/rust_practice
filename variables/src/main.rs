fn main() {
    let x = 5;

    let x = x + 1;

    {
        let x = x * 2;
        println!("The value of x in the inner scope is: {x}");
    }

    println!("The value of x is: {x}");

    operations();
    tuples();
    arrays();
}

fn operations() {
    // addition
    let _sum = 5 + 10;

    // subtraction
    let _difference = 95.5 - 4.3;

    // multiplication
    let _product = 4 * 30;

    // division
    let _quotient = 56.7 / 32.2;
    let _truncated = -5 / 3; // Results in -1

    // remainder
    let _remainder = 43 % 5;
}

fn tuples() {
    let tup = (500, 6.4, 1);

    // tuple destructuring
    let (_x, y,_zz) = tup;

    println!("The value of y is: {y}");

    // tuple indexing
    let _five_hundred = tup.0;

    let _six_point_four = tup.1;

    let _one = tup.2;

    let _unit = ();
}

fn arrays() {
    let a = [1, 2, 3, 4, 5]; // Arrays are always fixed in size
    let _three_five_times = [3; 5]; // [3, 3, 3, 3, 3]

    // array indexing
    let _first = a[0];
    let _second = a[1];
}