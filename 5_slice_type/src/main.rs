// Slice Type

fn first_word(s: &str) -> &str {
    let bytes:&[u8] = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        // First Word end here
        if item == b' ' {
            return &s[0..i];
        }
    }
    // All letters
    return &s[..];
}

fn main() {
    let phrase = "Hello, world!";
    let phrase_str = String::from(phrase);

    let word = first_word(&phrase[3..]);
    println!("{}, {}", phrase, word);

    let word = first_word(&phrase);
    println!("{}, {}", phrase, word);

    let word = first_word(&phrase_str); //coersion
    println!("{}, {}", phrase, word);
}
