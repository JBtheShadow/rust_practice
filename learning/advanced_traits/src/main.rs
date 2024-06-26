use std::ops::Add;
use std::fmt;

fn main() {
    placeholder_types_in_traits();
    generic_type_parameters_method_overload();
    calling_methods_same_name();
    supertraits();
    newtype_pattern();
}

fn placeholder_types_in_traits() {
    struct _Counter {}

    pub trait _Iterator {
        type Item;
    
        fn next(&mut self) -> Option<Self::Item>;
    }

    impl _Iterator for _Counter {
        type Item = u32;
    
        fn next(&mut self) -> Option<Self::Item> {
            Some(0)
        }
    }
}

fn generic_type_parameters_method_overload() {
    #[derive(Debug, Copy, Clone, PartialEq)]
    struct Point {
        x: i32,
        y: i32,
    }
    
    impl Add for Point {
        type Output = Point;
    
        fn add(self, other: Point) -> Point {
            Point {
                x: self.x + other.x,
                y: self.y + other.y,
            }
        }
    }

    assert_eq!(
        Point { x: 1, y: 0 } + Point { x: 2, y: 3 },
        Point { x: 3, y: 3 }
    );

    // Another example
    struct Millimeters(u32);
    struct Meters(u32);

    impl Add<Meters> for Millimeters {
        type Output = Millimeters;

        fn add(self, other: Meters) -> Millimeters {
            Millimeters(self.0 + (other.0 * 1000))
        }
    }
}

fn calling_methods_same_name() {
    trait Pilot {
        fn fly(&self);
    }
    
    trait Wizard {
        fn fly(&self);
    }
    
    struct Human;
    
    impl Pilot for Human {
        fn fly(&self) {
            println!("This is your captain speaking.");
        }
    }
    
    impl Wizard for Human {
        fn fly(&self) {
            println!("Up!");
        }
    }
    
    impl Human {
        fn fly(&self) {
            println!("*waving arms furiously*");
        }
    }

    let person = Human;
    
    // This defaults to Human's implementation of fly
    // it works like this because of the self parameter in fly, fly is a method
    person.fly();

    // You can do this to call the trait's version of each method
    Pilot::fly(&person);
    Wizard::fly(&person);

    // You can do this with Human's too
    Human::fly(&person);
    
    trait Animal {
        fn baby_name() -> String;
    }
    
    struct Dog;
    
    impl Dog {
        fn baby_name() -> String {
            String::from("Spot")
        }
    }
    
    impl Animal for Dog {
        fn baby_name() -> String {
            String::from("puppy")
        }
    }
    
    // Here, baby_name doesn't take a self so even if we instanced a dog we couldn't
    // access baby_name without fully qualifying the associated function
    println!("A baby dog is called a {}", Dog::baby_name());

    // And in this case if we wanted to call Animal's baby_name we are forced to do this
    println!("A baby dog is called a {}", <Dog as Animal>::baby_name());

    // Fully qualified syntax in general
    // <Type as Trait>::function(receiver_if_method, next_arg, ...);
}

fn supertraits() {
    // Basically trait requirements. If your trait relies on another trait
    // the other trait is called a supertrait

    // OutlinePrint requires fmt::Display
    trait OutlinePrint: fmt::Display {
        fn outline_print(&self) {
            let output = self.to_string();
            let len = output.len();
            println!("{}", "*".repeat(len + 4));
            println!("*{}*", " ".repeat(len + 2));
            println!("* {output} *");
            println!("*{}*", " ".repeat(len + 2));
            println!("{}", "*".repeat(len + 4));
        }
    }

    struct Point {
        x: i32,
        y: i32,
    }
    
    // Required impl for the OutlinePrint below
    impl fmt::Display for Point {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            write!(f, "({}, {})", self.x, self.y)
        }
    }

    // Wouldn't compile without the impl above
    impl OutlinePrint for Point {}

    let p = Point { x: 1, y: 2 };
    p.outline_print();
}

fn newtype_pattern() {
    // When we want to implement external traits on external types
    // For example, implementing Display on Vec<T>
    // By default Rust won't allow this because both are defined outside our crate/library
    // This is a way around that

    struct Wrapper(Vec<String>);

    impl fmt::Display for Wrapper {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            write!(f, "[{}]", self.0.join(", "))
        }
    }

    let w = Wrapper(vec![String::from("hello"), String::from("world")]);
    println!("w = {w}");
}
