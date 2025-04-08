// Функція для перевірки простоти числа
fn is_prime(n: u32) -> bool {
    if n < 2 {
        return false; // Числа менше 2 не є простими
    }
    for i in 2..=(n as f64).sqrt() as u32 {
        if n % i == 0 {
            return false; // Якщо є дільник, то число не просте
        }
    }
    true // Якщо немає дільників, то число просте
}

fn main() {
    // Тестування
    let test_numbers = [1, 2, 3, 4, 5, 16, 17, 18];
    
    for &num in test_numbers.iter() {
        println!("Число {} є простим? {}", num, is_prime(num));
    }
}
