#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

// All functions associated with the Rectangle struct go here
impl Rectangle {
    // This is a method/implementation, when a function is declared within the context of a struct or some other object
    // Methods have a self parameter
    fn area(&self) -> u32 {
        self.width * self.height
    }

    // This is also a method
    fn can_hold(&self, other_rect: &Rectangle) -> bool {
        self.width >= other_rect.width && self.height >= other_rect.height
    }

    // This isn't a method; it doesn't take self as a parameter
    // Often these functions are used as constructors
    fn square(size: u32) -> Self {
        Self {
            width: size,
            height: size,
        }
    }
}

// There can be multiple impl blocks too, apparently useful for some situations with generics
impl Rectangle {
    fn perimeter(&self) -> u32 {
        2 * self.height + 2 * self.width
    }
}

fn main() {
    base();
    multiple_parameters();
    associated_functions();
}

fn base() {
    let rect1 = Rectangle {
        width: 30,
        height: 50
    };

    // Can't print structs directly with Display unless you implement that trait
    //println!("rect1 is {}", rect1);

    // Can't print structs directl with Debug either unless you go back to the struct
    // and add the proper attribute
    //println!("rect1 is {:?}", rect1);

    // Can also use this to break the struct representation over multiple lines
    println!("rect1 is {:#?}", rect1);

    println!(
        "The area of the rectangle is {} square pixels.",
        rect1.area()
    );

    // Can also use the dbg! macro for printing stuff to the stderror instead of the stdout
    let scale = 2;
    let rect2 = Rectangle {
        width: dbg!(30 * scale),
        height: 50,
    };

    // dbg! takes ownership, unlike with println!, so use references for structs instead
    dbg!(&rect2);
}

fn multiple_parameters() {
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };
    let rect2 = Rectangle {
        width: 10,
        height: 40,
    };
    let rect3 = Rectangle {
        width: 60,
        height: 45,
    };

    println!("Can rect1 hold rect2? {}", rect1.can_hold(&rect2));
    println!("Can rect1 hold rect3? {}", rect1.can_hold(&rect3));
}

fn associated_functions() {
    // Associated functions can be called with the :: syntax,
    // just like with namespaces
    let sq = Rectangle::square(3);

    // While you can call methods via ::, it's less verbose to use
    // the . notation
    // This reminds me of extensions in C#
    let _perimeter_longhand = Rectangle::perimeter(&sq);
    let _perimeter_shorthand = sq.perimeter();
}