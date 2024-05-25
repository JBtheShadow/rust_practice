use std::fmt::Display;

use aggregator::{NewsArticle, Summary, Tweet};

// This struct holds a *reference*
// When this happens you also need to specify a lifetime for each reference
// "An instance of ImportantExcerpt can't outlive the reference to part"
struct ImportantExcerpt<'a> {
    part: &'a str,
}

impl<'a> ImportantExcerpt<'a> {
    fn level(&self) -> i32 {
        3
    }

    fn announce_and_return_part(&self, announcement: &str) -> &str {
        println!("Attention please: {}", announcement);
        self.part
    }
}

fn main() {
    generics();
    lifetimes();
    lifetimes_structs();
}

fn generics() {
    let tweet = Tweet {
        username: String::from("horse_ebooks"),
        content: String::from(
            "of course, as you probably already know, people",
        ),
        reply: false,
        retweet: false,
    };

    println!("1 new tweet: {}", tweet.summarize());

    let article = NewsArticle {
        headline: String::from("Penguins win the Stanley Cup Championship!"),
        location: String::from("Pittsburgh, PA, USA"),
        author: String::from("Iceburgh"),
        content: String::from(
            "The Pittsburgh Penguins once again are the best \
             hockey team in the NHL.",
        ),
    };

    println!("New article available! {}", article.summarize2());
}

fn lifetimes() {
    let string1 = String::from("abcd");
    let string2 = "xyz";

    let result = longest(string1.as_str(), string2);
    println!("The longest string is {}", result);

    let string1 = String::from("long string is long");

    {
        let string2 = String::from("xyz");
        let result = longest(string1.as_str(), string2.as_str());
        println!("The longest string is {}", result);
    }

    /*
    // This doesn't compile: the lifetime in the fn signature
    // EXPECTS the result to also be the smaller of the two
    // input references, which isn't true here;
    // result lives longer than string2
    let string1 = String::from("long string is long");
    let result;
    {
        let string2 = String::from("xyz");
        result = longest(string1.as_str(), string2.as_str());
    }
    println!("The longest string is {}", result);
    */
}

// 'a is a lifetime annotation
// In this case it's also being used as a generic lifetime
// because the lifetime is in the signature
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}
/*
// To solve this problem because without the lifetime annotations
// the compiler doesn't know how to resolve the borrow checker
// between x and y because two different references were passed 
// and the method return type also wants a reference
fn longest(x: &str, y: &str) -> &str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

// Lifetimes also only need to be specified for parameters
// with related lifetimes
// For example, if y does not relate to x or the result
// then y doesn't need the lifetime annotation
fn longest<'a>(x: &'a str, y: &str) -> &'a str {
    x
}

// The below doesn't compile because result doesn't actually relate
// to x nor y and it gets cleaned up after the function ends
// In this case it would be best to just return the owned object
// instead of a reference
fn longest<'a>(x: &str, y: &str) -> &'a str {
    let result = String::from("really long string");
    result.as_str()
}
*/

fn lifetimes_structs() {
    let novel = String::from("Call me Ishmael. Some years ago...");
    let first_sentence = novel.split('.').next().expect("Could not find a '.'");
    let i = ImportantExcerpt {
        part: first_sentence,
    };

    let _l = i.level();
    let _part = i.announce_and_return_part("An announcement!");

    let _result = longest_with_an_announcement("phrase A", "phrase B haha", "An announcement!");
}

/*
// String literals have a static lifetime
// which can be annotated like this
let s: &'static str = "I have a static lifetime.";

// Careful with static. The compiler might suggest to apply it
// to variables but see if it actually makes sense to do it
// or if the problem is a dangling reference that needs fixing instead
*/

// All in one example
fn longest_with_an_announcement<'a, T>(
    x: &'a str,
    y: &'a str,
    ann: T,
) -> &'a str
where
    T: Display,
{
    println!("Announcement! {}", ann);
    if x.len() > y.len() {
        x
    } else {
        y
    }
}