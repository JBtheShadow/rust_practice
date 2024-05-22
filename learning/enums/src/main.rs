enum IpAddrKind {
    V4,
    V6,
}

// Enums can have associated data as well,
// and they can be different types of data too
enum IpAddr {
    V4(u8, u8, u8, u8),
    V6(String),
}

enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter,
}

// Enums can have methods too
impl Message {
    fn call(&self) {
        // do stuff here
        match self {
            Self::Quit => println!("Quitting!"),
            Self::Move { x, y } => println!("Moving to [{x},{y}]"),
            Self::Write(s) => println!("{s}"),
            Self::ChangeColor(r, g, b) => println!("Changing color to ({r}, {g}, {b})"),
        }
    }
}

fn main() {
    ip_addresses();
    ip_addr_with_data();
    messages();
    option_enum();
    matching_enums();
    catch_all_and_placeholders();
    if_let();
}

fn ip_addresses() {
    let four = IpAddrKind::V4;
    let six = IpAddrKind::V6;

    route(four);
    route(six);
}

fn route(_ip_kind: IpAddrKind) {}

fn ip_addr_with_data() {
    let home = IpAddr::V4(127, 0, 0, 1);
    let loopback = IpAddr::V6(String::from("::1"));

    for addr in [home, loopback] {
        match addr {
            IpAddr::V4(a, b, c, d) => println!("Ipv4 address {}.{}.{}.{}", a, b, c, d),
            IpAddr::V6(s) => println!("Ipv6 address {}", s),
        }
    }
}

fn messages() {
    let w = Message::Write(String::from("hello"));
    let m = Message::Move { x: 50, y: 12 };
    let c = Message::ChangeColor(30, 30, 30);
    let q = Message::Quit;

    for message in [w, m, c, q] {
        message.call();
    }
}

fn option_enum() {
    let _some_number = Some(5);
    let _some_char = Some('e');

    let _absent_number: Option<i32> = None;
}

fn matching_enums() {
    let penny = Coin::Penny;
    let nickel = Coin::Nickel;
    let dime = Coin::Dime;
    let quarter = Coin::Quarter;
    
    println!("A penny is worth {} cent", value_in_cents(penny));
    println!("A nickel is worth {} cents", value_in_cents(nickel));
    println!("A dime is worth {} cents", value_in_cents(dime));
    println!("A quarter is worth {} cents", value_in_cents(quarter));

    let five = Some(5);
    let _six = plus_one(five);
    let _none = plus_one(None);
}

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter => 25,
    }
}

fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(i) => Some(i + 1),
    }
}

fn catch_all_and_placeholders() {
    let dice_roll = 9;
    match dice_roll {
        3 => add_fancy_hat(),
        7 => remove_fancy_hat(),
        other => move_player(other),
    }
}

fn add_fancy_hat() {}
fn remove_fancy_hat() {}
fn move_player(_num_spaces: u8) {}

fn if_let() {
    let config_max = Some(3u8);
    match config_max {
        Some(max) => println!("The maximum is configured to be {}", max),
        _ => println!("No maximum configured"),
    }

    if let Some(max) = config_max {
        println!("The maximum is configured to be {}", max);
    } else {
        println!("No maximum configured");
    }

}