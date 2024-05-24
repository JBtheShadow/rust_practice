use std::collections::HashMap;
use std::io;

enum CommandResult {
    Ok,
    Error(String),
}

enum Command {
    AddPersonToDepartment { name: String, department: String },
    ListByDepartment { department: String },
    ListAll,
    Quit,
    Error { error: String },
}

struct Database {
    employees: HashMap<String, Vec<String>>
}

impl Database {
    fn new() -> Self {
        Self { employees: HashMap::new() }
    }

    fn add_person(&mut self, name: &str, department: &str) -> CommandResult {
        let key = String::from(department);
        let value = String::from(name);
        let people = self.employees.entry(key).or_insert(Vec::new());
        if people.contains(&value) {
            return CommandResult::Error(format!("{name} is already in {department}!"));
        }
        people.push(value);
        people.sort();
        println!("Done.");
        return CommandResult::Ok;
    }

    fn list_department(&mut self, department: &str) -> CommandResult {
        let key = String::from(department);
        if !self.employees.contains_key(&key) {
            return CommandResult::Error(format!("{department} was not found!"));
        }
        println!("People in {department}:");
        let people = self.employees.entry(key).or_default();
        for person in people {
            println!("{}", person);
        }
        CommandResult::Ok
    }

    fn list_all(&mut self) -> CommandResult {
        let mut keys = Vec::new();
        for key in self.employees.keys().cloned() {
            keys.push(key);
        }

        if keys.len() < 1 {
            return CommandResult::Error(String::from("No people to list!"));
        }

        keys.sort();

        for key in keys {
            println!("People in {}:", &key);
            let people = self.employees.entry(key).or_default();
            for person in people {
                println!("{}", person);
            }
        }

        CommandResult::Ok
    }
}

fn main() {
    let mut db = Database::new();
    println!("Hello, operator!");
    loop {
        println!("Please input the desired operation. Available commands: add, list, quit");
        let input = get_input();
        let command = parse_command(&input);
        let result = match command {
            Command::AddPersonToDepartment { name, department } => db.add_person(&name, &department),
            Command::ListByDepartment { department } => db.list_department(&department),
            Command::ListAll => db.list_all(),
            Command::Error { error } => CommandResult::Error(error),
            Command::Quit => {
                println!("Thank you for your time, have a nice day!");
                break;
            }
        };
        match result {
            CommandResult::Error(error) => println!("{}", error),
            _ => (),
        }
    }
}

fn get_input() -> String {
    let mut input = String::new();
    loop {
        match io::stdin().read_line(&mut input) {
            Ok(_) => break input,
            _ => {
                println!("Unable to read command. Please try again.");
                continue
            }
        }
    }
}

fn parse_command(s: &str) -> Command {
    let mut parts = Vec::new();
    for word in s.split_whitespace() {
        parts.push(word);
    }

    if parts.len() < 1 || parts.len() > 4 {
        return Command::Error { error: String::from("Unknown command") };
    }

    let param1 = parts[0];
    let param2 = if parts.len() > 1 { Some(parts[1]) } else { None };
    let param3 = if parts.len() > 2 { Some(parts[2]) } else { None };
    let param4 = if parts.len() > 3 { Some(parts[3]) } else { None };

    match param1 {
        "add" | "Add" => {
            if param2.is_none() {
                return Command::Error { error: String::from("Unable to add. Command usage: Add [Person] to [Department]") };
            }
            let name = param2.unwrap();

            if param3.is_none() || param4.is_none() || param3.is_some_and(|s| s != "to" && s != "To") {
                return Command::Error { error: format!("Add {} to where? Command usage: Add [Person] to [Department]", name) };
            }

            let department = param4.unwrap();
            Command::AddPersonToDepartment { name: String::from(name), department: String::from(department) }
        }
        "list" | "List" => {
            if param3.is_some() || param4.is_some() {
                return Command::Error { error: String::from("Unable to list. Command usage:\r\nFor list all: List\r\nFor list by department: List [Department]") };
            }
            if param2.is_none() {
                Command::ListAll
            } else {
                let department = param2.unwrap();
                return Command::ListByDepartment { department: String::from(department) }
            }
        }
        "quit" | "Quit" => {
            if param2.is_some() || param3.is_some() || param4.is_some() {
                return Command::Error { error: String::from("To quit simply type: Quit") };
            }
            Command::Quit
        },
        _ => Command::Error { error: String::from("Unknown command") },
    }
}
