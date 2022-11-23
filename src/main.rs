fn main() {
    let numbers = 1..9;
    let characters = 'a'..'z';
    let special_characters: Vec<char> = format!("{}{}", "~`!@#$%^&*()_-+={[}]|:;'<,>./?", '"')
        .chars()
        .collect();
}
