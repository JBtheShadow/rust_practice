fn main() {
    // Patterns can be refutable (don't cover every case) or irrefutable (cover every case)

    patterns_on_if_lets();
    patterns_on_while_lets();
    patterns_on_for_loops();
    patterns_on_lets();
    patterns_on_function_parameters();

    pattern_syntax();
}

fn patterns_on_if_lets() {
    let favorite_color: Option<&str> = None;
    let is_tuesday = false;
    let age: Result<u8, _> = "34".parse();

    // Reminder: if let is a shorthand for match when only interested in one case
    if let Some(color) = favorite_color {
        println!("Using your favorite color, {color}, as the background");
    } else if is_tuesday {
        println!("Tuesday is green day!");
    } else if let Ok(age) = age {
        if age > 30 {
            println!("Using purple as the background color");
        } else {
            println!("Using orange as the background color");
        }
    } else {
        println!("Using blue as the background color");
    }
}

fn patterns_on_while_lets() {
    let mut stack = Vec::new();

    stack.push(1);
    stack.push(2);
    stack.push(3);

    while let Some(top) = stack.pop() {
        println!("{top}");
    }
}

fn patterns_on_for_loops() {
    let v = vec!['a', 'b', 'c'];

    for (index, value) in v.iter().enumerate() {
        println!("{value} is at index {index}");
    }
}

fn patterns_on_lets() {
    let _x = 5;
    let (_x, _y, _z) = (1, 2, 3);
}

fn patterns_on_function_parameters() {
    fn print_coordinates(&(x, y): &(i32, i32)) {
        println!("Current location: ({x}, {y})");
    }

    let point = (3, 5);
    print_coordinates(&point);
}

fn pattern_syntax() {
    // Literals
    let x = 1;

    match x {
        1 => println!("one"),
        2 => println!("two"),
        3 => println!("three"),
        _ => println!("anything"),
    }

    // Named variables
    let x = Some(5);
    let y = 10;

    match x {
        Some(50) => println!("Got 50"),
        Some(y) => println!("Matched, y = {y}"),
        _ => println!("Default case, x = {x:?}"),
    }

    println!("at the end: x = {x:?}, y = {y}");

    // Multiple cases
    let x = 1;

    match x {
        1 | 2 => println!("one or two"),
        3 => println!("three"),
        _ => println!("anything"),
    }
}
