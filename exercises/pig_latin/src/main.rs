fn main() {
    let inputs = get_inputs();
    for input in inputs {
        let output = pig_latin(&input);
        println!("'{}' becomes '{}'", input, output);
    }
}

fn get_inputs() -> Vec<String> {
    vec![
        String::from("first string"),
        String::from("second string"),
        String::from("a test string just to see what would happen to the text"),
    ]
}

fn pig_latin(s: &str) -> String {
    let mut phrase = String::new();
    for word in s.split_whitespace() {
        let mut first_letter: Option<char> = None;
        let mut latin_word = String::new();
        for letter in word.chars() {
            if first_letter.is_none() {
                first_letter = Some(letter);
                if is_vowel(letter) {
                    latin_word.push(letter);
                }
            } else {
                latin_word.push(letter);
            }
        }
        match first_letter {
            Some(c) if is_vowel(c) => latin_word.push_str("-hay"),
            Some(c) => {
                let s = format!("-{c}ay");
                latin_word.push_str(&s);
            }
            _ => (),
        }
        let s = format!(" {latin_word}");
        phrase.push_str(&s);
    }
    String::from(phrase.trim())
}

fn is_vowel(c: char) -> bool {
    "AEIOUaeiou".contains(c)
}