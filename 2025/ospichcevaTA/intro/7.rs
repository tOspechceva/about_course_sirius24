//Напишите программу, которая в последовательности чисел находит сумму двузначных чисел, кратных 8. Программа в первой строке получает на вход число n - количество чисел в последовательности, во второй строке -- n чисел, входящих в данную последовательность.
use super::common_functions;
use std::io;

pub fn task() -> io::Result<()> {
    let n: u32 = common_functions::input_int("Введите количество чисел n: ")?;
    
    let mut sum = 0i32; // используем i32, так как числа могут быть отрицательными
    
    for i in 0..n {
        let num: i32 = common_functions::input_int(&format!("Введите {}-е число: ", i + 1))?;
        
        // Проверяем, является ли число двузначным
        let abs_num = num.abs();
        if abs_num >= 10 && abs_num <= 99 {
            // Проверяем, делится ли число на 8
            if num % 8 == 0 {
                sum += num;
            }
        }
    }
    
    println!("Сумма двузначных чисел, кратных 8: {}", sum);
    
    Ok(())
}

// Вспомогательная функция для тестирования
pub fn sum_two_digit_multiples_of_8(numbers: &[i32]) -> i32 {
    numbers
        .iter()
        .filter(|&&x| {
            let abs_x = x.abs();
            abs_x >= 10 && abs_x <= 99 && x % 8 == 0
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sum_two_digit_multiples_of_8() {
        // Базовые случаи
        assert_eq!(sum_two_digit_multiples_of_8(&[]), 0);
        assert_eq!(sum_two_digit_multiples_of_8(&[1, 2, 3]), 0);
        
        // Двузначные числа, кратные 8
        assert_eq!(sum_two_digit_multiples_of_8(&[16, 24, 32]), 72);
        assert_eq!(sum_two_digit_multiples_of_8(&[8, 16, 100, 24]), 40); // 8 и 100 не двузначные
        
        // Отрицательные числа
        assert_eq!(sum_two_digit_multiples_of_8(&[-16, 24, -32]), -24);
        assert_eq!(sum_two_digit_multiples_of_8(&[-8, -16, -100]), -16); // -8 и -100 не двузначные
        
        // Смешанные случаи
        assert_eq!(sum_two_digit_multiples_of_8(&[10, 16, 20, 24, 30, 32, 40]), 112);
        assert_eq!(sum_two_digit_multiples_of_8(&[15, 17, 25, 33]), 0); // не кратны 8
        
        // Границы диапазона
        assert_eq!(sum_two_digit_multiples_of_8(&[10, 99, -10, -99]), 0); // не кратны 8
        assert_eq!(sum_two_digit_multiples_of_8(&[16, 96, -16, -96]), 0); // 16+96-16-96 = 0
    }
}