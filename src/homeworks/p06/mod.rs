fn main() {
    let triangles = 5; // Кількість трикутників в ялинці

    // Малюємо ялинку за допомогою iterator
    (1..=triangles).for_each(|i| {
        // Малюємо кожен трикутник
        (1..=i).for_each(|j| {
            let spaces = " ".repeat(triangles - i + j); // Визначаємо відступи
            let stars = "*".repeat(2 * j - 1); // Визначаємо кількість зірочок
            println!("{}{}", spaces, stars); // Виводимо кожен рядок трикутника
        });
    });

    // Малюємо стовпчик ялинки
    (0..2).for_each(|_| {
        let spaces = " ".repeat(triangles - 1);
        let trunk = "*".repeat(3);
        println!("{}{}", spaces, trunk);
    });
}
