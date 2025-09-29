// Сначала на вход поступает длина последовательности N. Затем элементы последовательности – целые числа. Напишите программу, которая подсчитывает количество положительных чисел среди элементов последовательности.

use super::common_functions;
use std::io;

pub fn task() -> io::Result<()> {
    let n: u32 = common_functions::input_int("Введите длину последовательности N: ")?;
    
    let mut count_positive = 0u32;
    
    for i in 0..n {
        let num: i32 = common_functions::input_int(&format!("Введите {}-е число: ", i + 1))?;
        if num > 0 {
            count_positive += 1;
        }
    }
    
    println!("Количество положительных чисел: {}", count_positive);
    
    Ok(())
}

// Вспомогательная функция для тестирования
pub fn count_positive_numbers(numbers: &[i32]) -> u32 {
    numbers.iter().filter(|&&x| x > 0).count() as u32
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_count_positive_numbers() {
        assert_eq!(count_positive_numbers(&[]), 0);
        assert_eq!(count_positive_numbers(&[1, 2, 3]), 3);
        assert_eq!(count_positive_numbers(&[-1, -2, -3]), 0);
        assert_eq!(count_positive_numbers(&[-1, 0, 1, 2]), 2);
        assert_eq!(count_positive_numbers(&[0, 0, 0]), 0);
        assert_eq!(count_positive_numbers(&[5, -3, 0, 7, -1]), 2);
    }
}