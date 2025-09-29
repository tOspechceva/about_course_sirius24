// Дано трехзначное число. Найдите сумму его цифр.
use super::common_functions;

use std::io;

pub fn task() -> io::Result<()> {
    let num = common_functions::input_int("трехзначное число.")?;
    let res = sum_of_digits(num);
    if res == None {
        println!("Ошибка: число должно быть трехзначным (100–999)!");
        return Ok(());
    }else {
        println!("Сумма цифр: {}", res.unwrap());
    }
    
    Ok(())
}

pub fn sum_of_digits(num: u32) -> Option<u32> {
    if num < 100 || num > 999 {
        None
    } else {
        let a = (num - num % 100) / 100;
        let b = (num - num % 10 - a * 100) / 10;
        let c = num % 10;
        Some(a + b + c)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sum_of_digits_valid() {
        assert_eq!(sum_of_digits(123), Some(6));   // 1+2+3
        assert_eq!(sum_of_digits(231), Some(6));   // 1+2+3
        assert_eq!(sum_of_digits(321), Some(6));   // 1+2+3
        assert_eq!(sum_of_digits(100), Some(1));   // 1+0+0
        assert_eq!(sum_of_digits(999), Some(27));  // 9+9+9
        assert_eq!(sum_of_digits(405), Some(9));   // 4+0+5
    }

    #[test]
    fn test_sum_of_digits_invalid() {
        assert_eq!(sum_of_digits(99), None);   // меньше 100
        assert_eq!(sum_of_digits(1000), None); // больше 999
        assert_eq!(sum_of_digits(0), None);    // меньше 100
    }
}