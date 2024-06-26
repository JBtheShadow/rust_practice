fn main() {
    newtype_pattern_safety_abstraction();
    type_aliases_to_solve_repetition();
    never_type();
    dynamically_sized_types();
}

fn newtype_pattern_safety_abstraction() {
    // They can be used to wrap values and give them
    // meaning while also reducing the margin of errors
    // in code because the compiler can enforce type checks

    struct _Meters(i32);
    struct _Millimeters(i32);
}

fn type_aliases_to_solve_repetition() {
    // This is not the same as a newtype, both names here are equivalent and the compiler will treat
    // both as the same
    type Kilometers = i32;

    let x: i32 = 5;
    let y: Kilometers = 5;

    println!("x + y = {}", x + y);

    // Instead, consider using aliases when the original type annotation makes the code verbose
    // and difficult to read and maintain
    type Thunk = Box<dyn Fn() + Send + 'static>;

    let f: Thunk = Box::new(|| println!("hi"));

    fn takes_long_type(_f: Thunk) {
        // --snip--
    }

    fn returns_long_type() -> Thunk {
        Box::new(|| println!("hello"))
    }

    takes_long_type(f);
    let _f = returns_long_type();

    // Type aliases can also have generics
    type _Result<T> = std::result::Result<T, std::io::Error>;
}

fn never_type() {
    // A function that panics never returns, it just aborts program execution
    fn bar() -> ! { panic!("This never returns/this has no return value!"); }
    
    // An infinite loop never returns so it's of the ! type
    fn infinite_loop() -> ! {
        print!("forever ");
        loop {
            print!("and ever ");
        }
    }

    // A continue inside of a loop doesn't actually return anything,
    // therefore it actually has the ! type
    for _ in [1..=3] {
        let guess = "32";

        // Inside of a match, all the arms must have the same return type
        // and any ! types found are coerced into whatever the only other type is
        // In this case it's u32
        let _guess: u32 = match guess.trim().parse() {
            Ok(843) => bar(),
            Ok(888) => infinite_loop(),
            Ok(num) => num,
            Err(_) => continue,
        };
    }
}

fn dynamically_sized_types() {
    // str is a dynamically sized type
}
