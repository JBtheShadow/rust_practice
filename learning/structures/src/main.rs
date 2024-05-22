// Regular struct
struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

// Tuple structs
struct Color(i32, i32, i32);
struct Point(i32, i32, i32);

// Unit-like structs
struct AlwaysEqual;

fn main() {
    let user1 = build_user(
        String::from("someone@example.com"),
        String::from("someusername123")
    );

    // Cannot mutate fields of a struct if the struct itself is immutable
    //user1.email = String::from("anotheremail@example.com");

    let mut user2 = build_user(
        String::from("someoneelse@example.com"),
        String::from("someusername456")
    );
    user2.email = String::from("anotheremail@example.com");

    // You can also use this struct update syntax to copy/move
    // all the non referenced fields
    let user3 = User {
        email: String::from("yetanotheremail@example.com"),
        ..user1
    };

    // For the example above, since username was one of the unlisted fields,
    // and String is a type with the Drop trait, the entire struct user1
    // can't be used anymore as a whole because ownership of the field
    // username was transferred

    // if user1.username == user3.username {
    //     println!("Same username!");
    // }

    // Fields with the Copy trait can still be accessed as normal though
    if user1.active {
        println!("User 1 is still active");
    }
    if user1.sign_in_count > 0 {
        println!("User 1 has signed in at least once");
    }

    if user3.username == "someusername123" {
        println!("Same username!");
    }

    // Tuple structs are considered different types even if parameters match
    let black = Color(0, 0, 0);
    let origin = Point(0, 0, 0);

    // They still allow for deconstructing and .index access
    let Color(_r, _g, _b) = black;
    let _x = origin.0;
    let _y = origin.1;
    let _z = origin.2;

    // Unit-like structs behave like ()
    let _subject = AlwaysEqual;

}

fn build_user(email: String, username: String) -> User {
    // You can use shorthand notation if the function parameters
    // and the struct fields have the same names
    User {
        active: true,
        username,
        email,
        sign_in_count: 1,
    }
}
