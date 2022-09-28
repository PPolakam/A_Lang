fn main() {
    println!("{}", evaluate(String::from("12.55")));
}

fn evaluate(line: String) -> f64 {
    let result = line.parse::<f64>();
    match result {
        Ok(number) => return number,
        _ => {}
    };

    // Check for scopes

    let by_scope = split_by_scope(line);

    

    0.0
}

fn split_by_scope(raw: String) -> Vec<String> {
    let mut tokens: Vec<String> = vec![String::new()];

    let openings = ['{', '[', '('];
    let closings = ['}', ']', ')'];

    let mut depth = 0;

    for character in raw.chars() {
        if openings.contains(&character) {
            tokens.push(String::new());
        } else if closings.contains(&character) {
            tokens.push(String::new());
        } else {
            let current = tokens.len() - 1;
            (&mut tokens[current]).push(character);
        }
    }

    tokens
}
