fn main() {
    let mut counter = 0;

    // You can assign values from loops this way
    let result = loop {
        counter += 1;

        if counter == 10 {
            // This stops the loop and returns the value counter * 2
            break counter * 2;
        }
    };

    println!("The result is {result}");

    loop_labels();
    conditional_loops();
    collection_loop_bad();
    collection_loop_good();
    countdown_with_for();
}

fn loop_labels() {
    let mut count = 0;
    // Loop labels start with a single quote
    'counting_up: loop {
        println!("count = {count}");
        let mut remaining = 10;

        loop {
            println!("remaining = {remaining}");
            if remaining == 9 {
                break; // stops innermost loop
            }
            if count == 2 {
                break 'counting_up; // stops labeled loop
            }
            remaining -= 1;
        }

        count += 1;
    }
    println!("End count = {count}");
}

fn conditional_loops() {
    let mut number = 3;

    while number != 0 {
        println!("{number}!");

        number -= 1;
    }
    println!("LIFTOFF!!!");
}

fn collection_loop_bad() {
    let a = [10, 20, 30, 40, 50];
    let mut index = 0;

    while index < 5 {
        println!("the value is: {}", a[index]);

        index += 1;
    }
}

fn collection_loop_good() {
    let a = [10, 20, 30, 40, 50];

    for element in a {
        println!("the value is: {element}");
    }
}

fn countdown_with_for() {
    for number in (1..4).rev() {
        println!("{number}!");
    }
    println!("LIFTOFF!!!");
}
