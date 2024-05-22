fn main() {
    variable_scope();
    string_type();
    variables_and_data_interacting_with_move();
    variables_and_data_interacting_with_clone();
    stack_only_data_copy();
    ownership_example();
    return_values_example();
    tuple_return();
    better_return();
    mutable_references();
    reference_restrictions();
    slices();
}

fn variable_scope() {
    // _s below isn't valid before it's declared or after it leaves
    // its {}'s scope
    {
        // This value is hardcoded directly into the executable
        let _s = "hello";

        // do stuff with _s
    }
}

fn string_type() {
    {
        // This value is allocated on the heap
        let mut s = String::from("hello");
        
        s.push_str(", world!");
        
        println!("{}", s);
    }
    // And deallocated automatically once its holding variable/owner
    // goes out of scope here
}

fn variables_and_data_interacting_with_move() {
    // Both of these values are pushed onto the stack
    let x = 5;
    let _y = x;

    // You'd think the same would happen here
    let s1 = String::from("hello");
    let _s2 = s1;
    // But actually after the line above, the ownership s1 had
    // was moved onto _s2 and s1 was invalidated
    // Trying to access s1 will generate a compile error 
}

fn variables_and_data_interacting_with_clone() {
    let s1 = String::from("hello");
    let s2 = s1.clone();

    // With clone now s2 is a copy of s1 and s1 remains valid
    println!("s1 = {}, s2 = {}", s1, s2);
}

fn stack_only_data_copy() {
    let x = 5;
    let y = x;

    // Since integers are primitive types, letting y = x creates a copy
    // of x, assigns it to y and places it on the stack. x remainx valid

    println!("x = {}, y = {}", x, y);
}

fn ownership_example() {
    // s comes into scope below
    let s = String::from("hello");  

    // s's value moves into the function...
    takes_ownership(s);
    // ...and so is no longer valid here

    // x comes into scope below
    let x = 5;

    // x would move into the function, but x has the copy trait
    // so instead a copy of x is made and x remains valid
    makes_copy(x);

    println!("{}", x);
}
// In reverse order, x goes out of scope, then s. Since s's ownership
// has already been moved, nothing happens to it

fn takes_ownership(some_string: String) {
    // some_string comes into scope above
    println!("{}", some_string);
}
// And here, some_string goes out of scope
// drop gets called and frees its memory

fn makes_copy(some_integer: i32) {
    // some_integer comes into scope above
    println!("{}", some_integer);
}
// And here, some_integer goes out of scope
// Nothing special happens

fn return_values_example() {
    // Functions that return a value transfer ownership back
    // to the variable they're being assigned to

    // Here s1 takes ownership of the String "yours"
    let s1 = gives_ownership();

    // Here s2 enters the scope
    let s2 = String::from("hello");

    // Here s2's ownership is moved into the function
    // and then returned back to s3
    let s3 = takes_and_gives_back(s2);

    // If I understand correctly then s1 and s3, at this point,
    // are valid while s2 has already been moved and cannot be used again
    // So println!("s1: {s1}, s2: {s2}, s3: {s3}"); won't compile
    println!("s1: {s1}, s3: {s3}");
}

fn gives_ownership() -> String {
    let some_string = String::from("yours");
    some_string
}

fn takes_and_gives_back(a_string: String) -> String {
    a_string
}

fn tuple_return() {
    let s1 = String::from("hello");

    // It is possible to return multiple values and among
    // them include the String data that was passed in,
    // as to allow it to be used further in the current function
    // There is a better way however
    let (s2, len) = calculate_length(s1);

    println!("The length of '{}' is {}.", s2, len);
}

fn calculate_length(s: String) -> (String, usize) {
    let length = s.len();

    (s, length)
}

fn better_return() {
    let s1 = String::from("hello");

    // Rather than passing the string itself, we pass a *reference*
    // to it. This looks a lot like c pointers at first glance
    let len = better_calculate_length(&s1);

    // The difference being a reference cannot point to an invalid
    // address while its scope is active

    println!("The length of '{}' is {}.", s1, len);
}

// Here we declare that the function accepts a reference to a String
// Same symbol rather than * and & for referencing and dereferencing
// Don't worry, though, * is still used for dereferencing
fn better_calculate_length(s: &String) -> usize {
    s.len()
}
// s only references a String, it doesn't own it. So nothing is dropped
// once it goes out of scope

// Creating a reference is also called borrowing

fn mutable_references() {
    let mut s = String::from("hello");

    change(&mut s);
}

// References are also immutable by default. To make them mutable,
// both the initial variable and the reference to it must be marked as such
fn change(some_string: &mut String) {
    some_string.push_str(", world");
}

fn reference_restrictions() {
    let mut s = String::from("hello");

    // There can be multiple immutable references at the same time
    let s1 = &s;
    let s2 = &s;

    // But you can't mix mutable and immutable ones within the same scope
    // This line below doesn't compile here
    //let sm= &mut s;

    println!("s1: {}, s2: {}", s1, s2);

    // It compiles here however because the compiler knows s1 and s2 are
    // last used before this declaration
    let _sm1= &mut s;

    // You also cannot have more than one mutable reference
    // to the same value within the same scope
    //let sm2 = &mut s;

    //println!("sm1: {}, sm2: {}", sm1, sm2);
}

fn slices() {
    let phrase = String::from("Hello world!");
    let _end = end_first_word(&phrase);
    // Here end has the value of 5, but if phrase were mutable
    // the value 5 might not be meaningful anymore were phrase
    // to change

    // For parts of a string we can use string slices
    let _hello = &phrase[0..5];
    let _world = &phrase[6..11];

    let _first = first_word(&phrase);
}

fn end_first_word(s: &String) -> usize {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return i;
        }
    }

    s.len()
}

// &str is for String slices
// A &String argument can be passed to a &str parameter with no problem, too
fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[..i];
        }
    }

    &s[..]
}

