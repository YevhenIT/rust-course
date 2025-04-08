// Функція для зміни регістру з верхнього на нижній
fn to_lowercase(input: &str) -> String {
    input.to_lowercase()
}

// Функція для зміни регістру з нижнього на верхній
fn to_uppercase(input: &str) -> String {
    input.to_uppercase()
}

fn main() {
    let text = "Hello, World!";
    
    // Зміна на нижній регістр
    let lower = to_lowercase(text);
    println!("Lowercase: {}", lower);
    
    // Зміна на верхній регістр
    let upper = to_uppercase(text);
    println!("Uppercase: {}", upper);
}
