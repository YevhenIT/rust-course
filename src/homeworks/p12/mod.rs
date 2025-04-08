// 1 Написати функцію яка рахує мінімальну кількість переносу грузу щоб на всіх кораблях був однаковий груз
pub fn count_permutation(shipments: &Vec<u32>) -> usize {
    let total: u32 = shipments.iter().sum();
    let n = shipments.len() as u32;

    if total % n != 0 {
        panic!("Неможливо рівномірно розподілити вантаж між кораблями");
    }

    let avg = total / n;
    let mut moves = 0;

    for &load in shipments {
        if load > avg {
            moves += (load - avg) as usize;
        }
    }

    moves
}

// 4 Альтернативна сигнатура у випадку, коли не завжди можливо
pub fn count_permutation(shipments: &Vec<u32>) -> usize {
    let total: u32 = shipments.iter().sum();
    let n = shipments.len() as u32;

    if total % n != 0 {
        panic!("Неможливо рівномірно розподілити вантаж між кораблями");
    }

    let avg = total / n;
    let mut moves = 0;

    for &load in shipments {
        if load > avg {
            moves += (load - avg) as usize;
        }
    }

    moves
}


// 5 Генерація коректного вектора вантажів
use rand::Rng;

pub fn gen_shipments(n: usize) -> Vec<u32> {
    let mut rng = rand::thread_rng();
    let avg = rng.gen_range(20..50); // середній вантаж
    let mut shipments: Vec<u32> = (0..n).map(|_| rng.gen_range(avg - 10..=avg + 10)).collect();

    // Підганяємо суму, щоб вона ділилася на n
    let sum: u32 = shipments.iter().sum();
    let rem = sum % (n as u32);

    if rem != 0 {
        shipments[0] += (n as u32 - rem);
    }

    shipments
}


//  Додати приклади з поясненнями
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_count_permutation() {
        let shipments = vec![10, 20, 30]; // avg = 20, перенос: з 30 -> 10 (10), з 10 не беремо
        assert_eq!(count_permutation(&shipments), 10);
    }

    #[test]
    #[should_panic]
    fn test_invalid_distribution() {
        let shipments = vec![10, 20, 31]; // 61 не ділиться на 3
        count_permutation(&shipments);
    }
}
