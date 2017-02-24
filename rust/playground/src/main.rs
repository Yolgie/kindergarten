fn main() {
    let word = "Blabidiblub";
    let mut answer = String::new();

    for char in word.chars() {
        answer.insert(0, char);
    }

    println!("{}", answer);

    println!("{}", reverse(&word));
    println!("{}", word);
}

fn reverse(&string: String) -> String {
    let mut word = string;
    word
}
