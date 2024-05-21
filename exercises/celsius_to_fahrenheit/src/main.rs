use std::io;

fn main() {
    let celsius = read_celsius();
    let fahrenheit = convert_to_fahrenheit(celsius);
    println!("{celsius}°C is {fahrenheit}°F");
}

fn read_celsius() -> f64 {
    loop {
        println!("Please input a temperature in Celsius:");
        
        let mut temp = String::new();

        match io::stdin().read_line(&mut temp) {
            Ok(_) => (),
            Err(_) => {
                println!("Failed to read line");
                continue;
            }
        };

        let value = match temp.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Invalid temperature");
                continue;
            }
        };

        break value;
    }
}

fn convert_to_fahrenheit(celsius: f64) -> f64 {
    celsius * 9.0 / 5.0 + 32.0
}
