// Convert strings to pig latin. The first consonant of each word is moved to
// the end of the word and “ay” is added, so “first” becomes “irst-fay.” Words
// that start with a vowel have “hay” added to the end instead (“apple” becomes
// “apple-hay”). Keep in mind the details about UTF-8 encoding!

fn main() {
    let converted = to_pig_latin(get_input());
    println!("{}", converted);
}

fn get_input() -> String {
    println!("Enter text:");
    let mut input = String::new();
    match std::io::stdin().read_line(&mut input) {
        Ok(_) => input.to_string(),
        Err(err) => panic!("Error: {}", err),
    }
}

// Checks only Latin vowels.
fn to_pig_latin(s: String) -> String {
    let vowels = "aeiouAEIOU";
    let mut converted: Vec<String> = Vec::new();
    for word in s.split_whitespace() {
        let mut is_first_char = true;
        let mut word_end = String::new();
        let mut word_beg = String::new();
        for c in word.chars() {
            if is_first_char {
                if vowels.contains(c) {
                    word_beg.push(c);
                    word_end = String::from("-hay");
                } else {
                    word_end = format!("-{}{}", c, "ay");
                }
                is_first_char = false;
            } else {
                word_beg.push(c);
            }
        }
        converted.push(word_beg + &word_end);
    }
    converted.join(" ")
}
