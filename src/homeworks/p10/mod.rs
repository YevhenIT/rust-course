// Функція для перевірки, чи є число паліндромом
fn is_palindrome(n: u32) -> bool {
    let original = n;
    let mut reversed = 0;
    let mut num = n;

    // Обчислення зворотного числа
    while num > 0 {
        reversed = reversed * 10 + num % 10;
        num /= 10;
    }

    // Перевірка, чи є число паліндромом
    original == reversed
}

fn main() {
    // Тестові числа
    let test_cases = [121, 12321, 123, 789, 1001];

    for &num in test_cases.iter() {
        if is_palindrome(num) {
            println!("{} є паліндромом", num);
        } else {
            println!("{} не є паліндромом", num);
        }
    }
}
