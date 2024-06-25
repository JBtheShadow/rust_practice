fn main() {
    // Patterns can be refutable (don't cover every case) or irrefutable (cover every case)

    patterns_on_if_lets();
    patterns_on_while_lets();
    patterns_on_for_loops();
    patterns_on_lets();
    patterns_on_function_parameters();
    pattern_syntax();
    use_on_destructuring();
    ignoring_parts_of_patterns();
    match_guards();
    at_bindings();
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

    // Combining patterns
    let x = 1;

    match x {
        1 | 2 => println!("one or two"),
        3 => println!("three"),
        _ => println!("anything"),
    }

    // Matching ranges
    let x = 5;

    match x {
        1..=5 => println!("one through five"),
        _ => println!("something else"),
    }

    let x = 'c';

    match x {
        'a'..='j' => println!("early ASCII letter"),
        'k'..='z' => println!("late ASCII letter"),
        _ => println!("something else"),
    }
}

fn use_on_destructuring() {
    //Structs
    struct Point {
        x: i32,
        y: i32,
    }

    let p = Point { x: 0, y: 7 };

    // Destructure Point(x,y) into values a and b to use them separately
    let Point { x: a, y: b } = p;
    assert_eq!(0, a);
    assert_eq!(7, b);

    // Shorthand notation when variables names match field ones
    let Point { x, y } = p;
    assert_eq!(0, x);
    assert_eq!(7, y);

    let p = Point { x: 0, y: 7 };

    // Destructuring with literal values for separate matches
    match p {
        Point { x: 0, y: 0 } => println!("On the origin"),
        Point { x, y: 0 } => println!("On the x axis at {x}"),
        Point { x: 0, y } => println!("On the y axis at {y}"),
        Point { x, y } => {
            println!("On neither axis: ({x}, {y})");
        }
    }

    // Enums
    enum Message {
        Quit,
        Move { x: i32, y: i32 },
        Write(String),
        ChangeColor(i32, i32, i32),
    }

    let msgs = vec![
        Message::Quit,
        Message::Move { x: 2, y: 3 },
        Message::Write(String::from("Some message here")),
        Message::ChangeColor(0, 160, 255)
    ];

    fn test_msg(msg: &Message) {
        match msg {
            Message::Quit => {
                println!("The Quit variant has no data to destructure.");
            }
            Message::Move { x, y } => {
                println!("Move in the x direction {x} and in the y direction {y}");
            }
            Message::Write(text) => {
                println!("Text message: {text}");
            }
            Message::ChangeColor(r, g, b) => {
                println!("Change the color to red {r}, green {g}, and blue {b}")
            }
        }
    }

    for msg in msgs.iter() {
        test_msg(msg)
    }

    // Nested structs and enums
    enum Color {
        _Rgb(i32, i32, i32),
        Hsv(i32, i32, i32),
    }
    
    enum Message2 {
        _Quit,
        _Move { x: i32, y: i32 },
        _Write(String),
        ChangeColor(Color),
    }

    let msg = Message2::ChangeColor(Color::Hsv(0, 160, 255));

    match msg {
        Message2::ChangeColor(Color::_Rgb(r, g, b)) => {
            println!("Change color to red {r}, green {g}, and blue {b}");
        }
        Message2::ChangeColor(Color::Hsv(h, s, v)) => {
            println!("Change color to hue {h}, saturation {s}, value {v}")
        }
        _ => (),
    }
    
    // Structs and tuples
    let ((feet, inches), Point { x, y }) = ((3, 10), Point { x: 3, y: -10 });
    assert_eq!(feet, 3);
    assert_eq!(inches, 10);
    assert_eq!(x, 3);
    assert_eq!(y, -10);
}

fn ignoring_parts_of_patterns() {
    // _ to ignore values completely
    fn foo(_: i32, y: i32) {
        println!("This code only uses the y parameter: {y}");
    }
    foo(3, 4);

    // Ignore parts of a value with a nested _
    let mut setting_value = Some(5);
    let new_setting_value = Some(10);

    match (setting_value, new_setting_value) {
        (Some(_), Some(_)) => {
            println!("Can't overwrite an existing customized value");
        }
        _ => {
            setting_value = new_setting_value;
        }
    }

    println!("setting is {setting_value:?}");

    // _ to ignore parts of a pattern
    let numbers = (2, 4, 8, 16, 32);

    match numbers {
        (first, _, third, _, fifth) => {
            println!("Some numbers: {first}, {third}, {fifth}")
        }
    }

    // _ in variable names to avoid compile warnings on unused variables
    let _s = String::from("Some string I'm not using");

    // .. to "ignore the rest", basically fill the remaining match with as many _'s as needed
    struct Point3 {
        x: i32,
        _y: i32,
        _z: i32,
    }

    let origin = Point3 { x: 0, _y: 0, _z: 0 };

    match origin {
        Point3 { x, .. } => println!("x is {x}"),
    }

    // Another example, now in the middle of a tuple
    let numbers = (2, 4, 8, 16, 32);

    match numbers {
        (first, .., last) => {
            println!("Some numbers: {first}, {last}");
        }
    }

}

fn match_guards() {
    let num = Some(4);

    match num {
        Some(x) if x % 2 == 0 => println!("The number {x} is even"),
        Some(x) => println!("The number {x} is odd"),
        None => (),
    }

    // Another example
    let x = Some(5);
    let y = 10;

    match x {
        Some(50) => println!("Got 50"),
        Some(n) if n == y => println!("Matched, n = {n}"),
        _ => println!("Default case, x = {x:?}"),
    }

    println!("at the end: x = {x:?}, y = {y}");

    // Match guards apply to the entire pattern
    let x = 4;
    let y = false;

    // So            4 | 5 |  6  if y => ...
    // behaves like (4 | 5 |  6) if y => ...
    // and not       4 | 5 | (6  if y) => ...
    match x {
        4 | 5 | 6 if y => println!("yes"),
        _ => println!("no"),
    }
}

fn at_bindings() {
    enum Message {
        Hello { id: i32 },
    }

    let msg = Message::Hello { id: 5 };

    // In the first and second arms, id: 3..=7 and id: 10..=12 are both pattern matches and cannot be used directly
    // If we wanted to use the found value we use the @ binding. It lets one test a value and capture it for use
    match msg {
        Message::Hello {
            id: id_variable @ 3..=7,
        } => println!("Found an id in range: {id_variable}"),
        Message::Hello { id: 10..=12 } => {
            println!("Found an id in another range")
        }
        Message::Hello { id } => println!("Found some other id: {id}"),
    }
}
