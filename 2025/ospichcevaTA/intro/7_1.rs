// Последовательность состоит из натуральных чисел и завершается числом 0. Определите количество элементов этой последовательности, которые равны ее наибольшему элементу.
use super::common_functions;
use std::io;

pub fn task() -> io::Result<()> {
    let mut numbers = Vec::new();
    
    println!("Введите последовательность натуральных чисел, завершающуюся нулём:");
    
    loop {
        let num: u32 = common_functions::input_int("Введите число: ")?;
        if num == 0 {
            break;
        }
        numbers.push(num);
    }
    
    if numbers.is_empty() {
        println!("Последовательность пуста (сразу введён 0).");
        return Ok(());
    }
    
    // Находим максимальный элемент
    let max_value = *numbers.iter().max().unwrap();
    
    // Считаем, сколько раз встречается максимальный элемент
    let count = numbers.iter().filter(|&&x| x == max_value).count();
    
    println!("Количество элементов, равных максимальному ({}): {}", max_value, count);
    
    Ok(())
}

// Вспомогательная функция для тестирования
pub fn count_max_occurrences(numbers: &[u32]) -> Option<usize> {
    if numbers.is_empty() {
        return None;
    }
    let max_val = *numbers.iter().max()?;
    let count = numbers.iter().filter(|&&x| x == max_val).count();
    Some(count)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_count_max_occurrences() {
        assert_eq!(count_max_occurrences(&[]), None);
        assert_eq!(count_max_occurrences(&[1]), Some(1));
        assert_eq!(count_max_occurrences(&[1, 2, 3]), Some(1));
        assert_eq!(count_max_occurrences(&[3, 2, 3, 1, 3]), Some(3));
        assert_eq!(count_max_occurrences(&[5, 5, 5]), Some(3));
        assert_eq!(count_max_occurrences(&[10, 3, 7, 10, 2, 10]), Some(3));
    }
}