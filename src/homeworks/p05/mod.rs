// Функція для обчислення найбільшого спільного дільника (GCD) двох чисел
fn gcd(a: u32, b: u32) -> u32 {
    let mut a = a;
    let mut b = b;

    // Алгоритм Евкліда
    while b != 0 {
        let temp = b;
        b = a % b;
        a = temp;
    }
    
    a // Повертаємо GCD
}

fn main() {
    // Приклад використання функції gcd
    let num1 = 56;
    let num2 = 98;

    println!("GCD of {} and {} is {}", num1, num2, gcd(num1, num2));
}
