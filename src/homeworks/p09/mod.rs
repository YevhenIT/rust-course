// Функція для зсуву рядка на задану кількість символів
fn shift_string(s: &str, shift: usize) -> String {
    let len = s.len();
    if len == 0 {
        return s.to_string(); // Якщо рядок порожній, повертаємо його без змін
    }

    let shift = shift % len; // Зсув не може бути більшим за довжину рядка
    let (first, second) = s.split_at(len - shift); // Розбиваємо рядок на дві частини
    format!("{}{}", second, first) // Переставляємо частини
}

fn main() {
    // Тест кейси
    let test_cases = [
        ("abcdef", 2),  // Зсунути на 2 позиції
        ("rust", 1),    // Зсунути на 1 позицію
        ("hello", 5),   // Зсунути на довжину рядка (повернути той самий рядок)
        ("world", 3),   // Зсунути на 3 позиції
    ];

    for (input, shift) in test_cases.iter() {
        println!("Original: '{}', Shift: {}, Result: '{}'", input, shift, shift_string(input, *shift));
    }
}
