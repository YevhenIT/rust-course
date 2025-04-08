use rand::Rng;  // Для генерації випадкових чисел

// Функція для генерування рандомного вектора довжиною n з чисел в діапазоні [10..99]
fn gen_random_vector(n: usize) -> Vec<i32> {
    let mut rng = rand::thread_rng();
    (0..n).map(|_| rng.gen_range(10..100)).collect()
}

// Функція для знаходження мінімальної пари в векторі
fn min_adjacent_sum(data: &[i32]) -> i32 {
    // Перевірка, чи вектор містить хоча б два елементи
    if data.len() < 2 {
        panic!("Вектор повинен містити хоча б два елементи");
    }

    // Ініціалізація мінімальної суми з першої пари
    let mut min_sum = data[0] + data[1];

    // Проходимо через всі пари сусідніх елементів
    for i in 1..data.len() - 1 {
        let sum = data[i] + data[i + 1];
        if sum < min_sum {
            min_sum = sum;
        }
    }

    min_sum
}

// Функція для виведення результату
fn print_result(data: &[i32]) {
    println!("Генерація випадкового вектора: {:?}", data);
    let min_sum = min_adjacent_sum(data);
    println!("Мінімальна сума сусідніх елементів: {}", min_sum);
}

fn main() {
    // Генеруємо випадковий вектор довжиною 20
    let random_vector = gen_random_vector(20);

    // Виводимо результат
    print_result(&random_vector);
}
