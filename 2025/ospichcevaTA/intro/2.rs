//Дано трехзначное число. Переверните его, а затем выведите.
use super::common_functions;

use std::io;

pub fn task() -> io::Result<()> {
    let num = common_functions::input_int("трехзначное число.")?;
    let res = revers_num(num);
    if res == None {
        println!("Ошибка: число должно быть трехзначным (100–999)!");
        return Ok(());
    }else {
        println!("Перевернутое число: {}", res.unwrap());
    }
    
    Ok(())
}
pub fn revers_num(num: u32) -> Option<u32> {
    if num < 100 || num > 999 {
        None
    } else {
        let a = (num - num % 100) / 100;
        let b = (num - num % 10 - a * 100) / 10;
        let c = num % 10;
        Some(c*100 + b*10 + a)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_revers_num_valid() {
        assert_eq!(revers_num(123), Some(321));   
        assert_eq!(revers_num(100), Some(1));   
        assert_eq!(revers_num(999), Some(999));  
        assert_eq!(revers_num(405), Some(504));   
    }

    #[test]
    fn test_revers_num_invalid() {
        assert_eq!(revers_num(99), None);   // меньше 100
        assert_eq!(revers_num(1000), None); // больше 999
        assert_eq!(revers_num(0), None);    // меньше 100
    }
}