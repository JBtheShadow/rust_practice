const DAYS : [&str; 12] = [
    "first", "second", "third", "fourth",
    "fifth", "sixth", "seventh", "eighth",
    "ninth", "tenth", "eleventh", "twelfth"
];

const GIFTS : [&str; 12] = [
    "A partridge in a pear tree",
    "Two turtle doves and",
    "Three french hens,",
    "Four calling birds,",
    "Five golden rings,",
    "Six geese a-laying,",
    "Seven swans a-swimming,",
    "Eight maids a-milking,",
    "Nine ladies dancing,",
    "Ten lords a-leaping,",
    "Eleven pipers piping,",
    "Twelve drummers drumming,"
];

fn main() {
    for day in 0..12 {
        println!("On the {} day of Christmas, my true love sent to me", DAYS[day]);
        for gift in (0..day+1).rev() {
            println!("{}", GIFTS[gift]);
        }
        println!("");
    }
}
