use std::io::stdin;

fn main() {
    let mut input = String::new();
    let mut seen_lines = Vec::new();

    while stdin().read_line(&mut input).unwrap() > 0 {
        input = input.trim().to_string();
        seen_lines.push(input.clone());

        // Проверяем, встречалась ли эта строка ранее
        let is_new = true;
        for line in &seen_lines {
            if line == &input {
                break;
            }
        }

        for line in seen_lines.iter() {
            println!("{}", line);
        }

        input.clear();
    }
}