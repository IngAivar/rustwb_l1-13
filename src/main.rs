use std::io::{self, BufRead};

fn main() {
    
    let stdin = io::stdin();
    let mut lines = Vec::new();

    // Чтение строк из стандартного потока ввода
    for line in stdin.lock().lines() {
        let line = line.unwrap();

        // Проверка на пустую строку для завершения программы
        if line.is_empty() {
            break;
        }

        // Если строка еще не встречалась, добавляем ее в вектор
        if !lines.contains(&line) {
            lines.push(line);
        }
    }

     println!("_________Уникальные строки_________");

    // Вывод уникальных строк
    for line in lines {
        println!("{}", line);
    }
}