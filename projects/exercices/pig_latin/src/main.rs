fn main() {
    let my_string_vowel = String::from("apple");
    let my_string_consonant = String::from("first");

    fn pig_latin(s: &String) -> String {
        let vowels: [char; 10] = ['a', 'e', 'i', 'o', 'u', 'A', 'E', 'I', 'O', 'U'];
        let consonants: [char; 42] = [
            'b', 'c', 'd', 'f', 'g', 'h', 'j', 'k', 'l', 'm', 'n', 'p', 'q', 'r', 's', 't', 'v',
            'w', 'x', 'y', 'z', 'B', 'C', 'D', 'F', 'G', 'H', 'J', 'K', 'L', 'M', 'N', 'P', 'Q',
            'R', 'S', 'T', 'V', 'W', 'X', 'Y', 'Z',
        ];
        //First char
        let first_char = s.chars().next().unwrap();

        //New String to be altered and returned
        let mut new_s = String::new();

        if consonants.contains(&first_char) {
            println!("First char is a consonant");
            new_s = String::from(&s[1..]);
            new_s.push('-');
            new_s.push(first_char);
            new_s.push_str("ay");
        }

        if vowels.contains(&first_char) {
            println!("First char is a vowel");
            new_s = s.clone();
            new_s.push_str("-hay");
        }

        new_s
    }

    println!("{}", pig_latin(&my_string_consonant));
    println!("{}", pig_latin(&my_string_vowel));
}
