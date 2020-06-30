// Convert strings to pig latin. The first consonant of each word is moved to the end of the word and “ay” is added, so “first” becomes “irst-fay.” Words that start with a vowel have “hay” added to the end instead (“apple” becomes “apple-hay”). Keep in mind the details about UTF-8 encoding!

fn pigify (s: &str) -> String {
    let mut chars = s.chars();
    let first_char:char = chars.next().unwrap();
    
    match first_char {
        'a' | 'e' | 'i' | 'o' | 'u' => format!("{}-hay", s),
        _ => format!("{}-{}ay", chars.as_str(), first_char),
    }
}

fn main() {
    let s = String::from("apple");
    println!("converted {:?}", pigify(&s));
}
