//Дано пятизначное число. Найдите произведение его цифр.
use super::common_functions;
use std::io;

pub fn task() -> io::Result<()> {
    let num = common_functions::input_int("пятизначное число.")?;
    let res = mul_of_digits(num);
    if res == None {
        println!("Ошибка: число должно быть пятизначным (10000–99999)!");
        return Ok(());
    }else {
        println!("Произведение цифр: {}", res.unwrap());
    }
    
    Ok(())
}

pub fn mul_of_digits(num: u32) -> Option<u32> {
    if num < 10000 || num > 99999 {
        None
    } else {
        let d = (num - num % 10000) / 10000;
        let e = (num - num % 1000 - d * 10000) / 1000;
        let a = (num - num % 100 - d * 10000 - e * 1000) / 100;
        let b = (num - num % 10 - a * 100  - d*10000 - e * 1000) / 10;
        let c = num % 10;
        Some(a * b * c * d * e)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mul_of_digits_valid() {
        assert_eq!(mul_of_digits(11111), Some(1));   
        assert_eq!(mul_of_digits(12345), Some(120));   
        assert_eq!(mul_of_digits(99999), Some(59049));  
        assert_eq!(mul_of_digits(10000), Some(0));   
    }

    #[test]
    fn test_mul_of_digits_invalid() {
        assert_eq!(mul_of_digits(9999), None);   // меньше 10000
        assert_eq!(mul_of_digits(100000), None); // больше 99999
        assert_eq!(mul_of_digits(0), None);    // меньше 10000
    }
}