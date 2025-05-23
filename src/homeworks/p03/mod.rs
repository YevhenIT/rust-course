fn main() {
    // Встановлюємо розміри конверту
    const W: usize = 30; // Ширина
    const H: usize = 15; // Висота

    // Малюємо верхню частину конверту
    for i in 0..W {
        print!("*");
    }
    println!(); // Перехід на новий рядок

    // Малюємо середню частину конверту (стіни та діагональні лінії)
    for row in 0..H - 2 {
        // Малюємо ліву частину конверту
        print!("*");

        // Малюємо пробіли для діагональної лінії
        for col in 0..(W - 2 * (row + 1)) {
            print!(" ");
        }

        // Малюємо праву частину конверту
        print!("*");

        // Перехід на новий рядок після кожного ряду
        println!();
    }

    // Малюємо нижню частину конверту
    for i in 0..W {
        print!("*");
    }
    println!(); // Перехід на новий рядок
}
