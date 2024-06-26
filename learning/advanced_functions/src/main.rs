fn main() {
    function_pointers();
    returning_closures();
}

fn function_pointers() {
    // Fn is a closure trait while fn is a function pointer type
    // fn also implements all three closure traits Fn, FnMut and FnOnce
    // so it can be used anywhere either of those three traits are expected

    fn add_one(x: i32) -> i32 {
        x + 1
    }

    fn do_twice(f: fn(i32) -> i32, arg: i32) -> i32 {
        f(arg) + f(arg)
    }

    let answer = do_twice(add_one, 5);

    println!("The answer is: {answer}");

    // Best practice is to implement methods/functions to require the proper closure trait
    // so that both closures and functions can be used
    // One exception is if the function interfaces with code that doesn't have closures, like external C code

    let list_of_numbers = vec![1, 2, 3];
    
    // Calling .map using a closure
    let _list_of_strings: Vec<String> =
        list_of_numbers.iter().map(|i| i.to_string()).collect();
    
    // Calling .map using a function/method
    let _list_of_strings_again: Vec<String> =
        list_of_numbers.iter().map(ToString::to_string).collect();
    
    // Another example using enum names
    enum Status {
        Value(u32),
        _Stop,
    }

    let list_of_statuses: Vec<Status> = (0u32..20).map(Status::Value).collect();
    for status in list_of_statuses {
        if let Status::Value(a) = status {
            println!("Value {}", a);
        }
    }
}

fn returning_closures() {
    type ClosureOfIntToInt = dyn Fn(i32) -> i32;

    // Because closures are traits you can't return them directly from functions.
    // Nothing's stopping you from boxing them, however

    fn returns_closure() -> Box<ClosureOfIntToInt> {
        Box::new(|x| x + 1)
    }

    let closure_box = returns_closure();

    // Then Rust allows you to use this boxed closure as if it were the closure itself or a function
    // letting you call it with parameters
    let result = closure_box(3);
    println!("The result is {}", result);
}
