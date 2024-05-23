enum SpreadsheetCell {
    Int(i32),
    Float(f64),
    Text(String),
}

fn main() {
    vectors();
    strings();
    hashmaps();
}

// Equivalent to lists in C#
fn vectors() {
    // Instantiating
    let _v: Vec<i32> = Vec::new();

    let _v = vec![1, 2, 3];

    // Adding values to it
    let mut v = Vec::new();

    v.push(5);
    v.push(6);
    v.push(7);
    v.push(8);

    // Accessing values
    let v = vec![1, 2, 3, 4, 5];

    let third: &i32 = &v[2];
    println!("The third element is {third}");

    let third: Option<&i32> = v.get(2);
    match third {
        Some(third) => println!("The third element is {third}"),
        None => println!("There is no third element."),
    }

    // Reminder on mutable and immutable references
    let mut v = vec![1, 2, 3, 4, 5];

    let first = &v[0];
    // This line doesn't compile here
    //v.push(9);
    println!("The first element is: {first}");

    // But compiles here if that first isn't used anymore
    v.push(9);

    // Iterating
    let v = vec![100, 32, 57];
    for i in &v {
        println!("{i}")
    }

    // Iterating + dereferencing
    let mut v = vec![100, 32, 57];
    for i in &mut v {
        *i += 50;
    }

    // Vectors of enums to store multiple types
    let row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Text(String::from("blue")),
        SpreadsheetCell::Float(10.12),
    ];
    for cell in &row {
        match cell {
            SpreadsheetCell::Int(i) => println!("{}", i),
            SpreadsheetCell::Float(f) => println!("{}", f),
            SpreadsheetCell::Text(t) => println!("{}", t),
        }
    }

    // Dropping a vector drops its elements
    {
        let _v = vec![1, 2, 3, 4];

        // do stuff with _v
    } // <- _v goes out of scope and is freed here
}

// Strings are actually implemented as a collection in Rust,
// so many of the operations that work with vectors also work
// with strings
fn strings() {
    // Creating empty
    let mut _s = String::new();

    // Creating with some data
    let data = "initial contents";
    let _s = data.to_string();

    let _s = "initial contents".to_string();

    let _s = String::from("initial contents");

    // Strings are UTF-8 encoded
    let _hello = String::from("السلام عليكم");
    let _hello = String::from("Dobrý den");
    let _hello = String::from("Hello");
    let _hello = String::from("שָׁלוֹם");
    let _hello = String::from("नमस्ते");
    let _hello = String::from("こんにちは");
    let _hello = String::from("안녕하세요");
    let _hello = String::from("你好");
    let _hello = String::from("Olá");
    let _hello = String::from("Здравствуйте");
    let _hello = String::from("Hola");

    // Updating a string
    let mut s = String::from("foo");
    s.push_str("bar");

    // push_str takes a string slice (&str) as to not take ownership of the parameter
    let mut s1 = String::from("foo");
    let s2 = "bar";
    s1.push_str(s2);
    println!("s2 is {s2}");

    // We can also push a single character
    let mut s = String::from("lo");
    s.push('l');

    // Concatenation with +
    let s1 = String::from("Hello, ");
    let s2 = String::from("world!");
    // the add operator is defined as taking ownership of the lefthand side,
    // appending the righthand side to it via reference
    // and returning the concatenated result
    let _s3 = s1 + &s2;
    
    // this gets unwieldy if needing to join multiple strings
    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");

    let _s = s1 + "-" + &s2 + "-" + &s3;

    // in cases like that, instead, use format!
    // format! also doesn't take ownership so the strings
    // can still be used afterwards
    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");

    let _s = format!("{s1}-{s2}-{s3}");

    // Indexing
    let _s1 = String::from("hello");
    // This line doesn't compile; Strings in Rust don't support indexing like that
    //let h = _s1[0];

    // Internally, Strings are a wrapper over a Vec<u8>
    // But because characters have a variable size, Rust decided not to allow indexing strings directly at all
    let _hello = String::from("Hola"); // len 3
    let _hello = String::from("Здравствуйте"); // len 24, not 12
    
    // There are actually three relevant ways to look at Strings, from Rust's perspective
    // Consider the Hindi word नमस्ते written in the Devanagari script
    // It can be viewed as bytes, with len 18:
    let _s = [224, 164, 168, 224, 164, 174, 224, 164, 184, 224, 165, 141, 224, 164, 164, 224, 165, 135];
    // It can be viewed as Unicode scalar values, Rust's char type, len 6:
    let _s = ['न', 'म', 'स', '्', 'त', 'े'];
    // Or it can be viewed as grapheme clusters, what humans would perceive the four letters as:
    let _s = ["न", "म", "स्", "ते"];

    // Careful when slicing strings
    let hello = "Здравствуйте";

    let _s = &hello[0..4]; // Зд

    // Rust panics at runtime if trying to slice a string at an invalid position
    // For example, trying to get half of the character З
    //let _s = &hello[0..1];

    // You can iterate over strings too
    let s = "Зд";
    // over chars
    for c in s.chars() {
        println!("{c}");
    }
    // over bytes
    for b in s.bytes() {
        println!("{b}");
    }
    // Just remember valid Unicode scalar values may be made up of more than 1 byte
    // Getting grapheme clusters isn't as simple so they aren't included in the standard
    // lib. There are crates/libs one can use for them if required

}

use std::collections::HashMap;

// Similar to dictionaries
// Rust's HashMap uses an implementation called SipHash that provides resistance to DoS attacks involving hash tables
fn hashmaps() {
    // Creation
    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);

    // Accessing values
    let team_name = String::from("Blue");
    let score = scores.get(&team_name).copied().unwrap_or(0);
    println!("{}'s score is {}", team_name, score);

    // Iterating through values
    for (key, value) in &scores {
        println!("{key}: {value}");
    }

    // Hashmap takes ownership of drop trait values
    let field_name = String::from("Favorite color");
    let field_value = String::from("Blue");

    let mut map = HashMap::new();
    map.insert(field_name, field_value);
    // At this point field_name and field_value have been moved and cannot be accessed anymore

    // Updating values
    // via overwriting
    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Blue"), 25); // 25 overwrites the value 10

    println!("{:?}", scores);
    
    //via insert only if not present
    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);

    scores.entry(String::from("Yellow")).or_insert(50); // inserts
    scores.entry(String::from("Blue")).or_insert(50); // ignores

    println!("{:?}", scores);

    //via combining old and new values
    let text = "hello world wonderful world";

    let mut map = HashMap::new();

    for word in text.split_whitespace() {
        let count = map.entry(word).or_insert(0);
        *count += 1;
    }

    println!("{:?}", map);
}
