//Дано шестизначное число. Найдите суммы его четных и нечетных элементов. Образуйте из них этих сумм оддно число и выведите его на экран
use super::common_functions;
use std::io;

pub fn task() -> io::Result<()> {
    let num = common_functions::input_int("шестизначное число.")?;
    let res = sum_even_odd(num);
    if res == None {
        println!("Ошибка: число должно быть шестизначным (100000–999999)!");
        return Ok(());
    }else {
        println!("Число из сумм четных и нечетных элементов: {}", res.unwrap());
    }
    
    Ok(())
}

pub fn sum_even_odd(num: u32) -> Option<String> {
    if num < 100000 || num > 999999 {
        None
    } else {
        let f = (num - num % 100000) / 100000;
        let d = (num - num % 10000 - f * 100000) / 10000;
        let e = (num - num % 1000 - d * 10000 - f * 100000) / 1000;
        let a = (num - num % 100 - d * 10000 - e * 1000 - f * 100000) / 100;
        let b = (num - num % 10 - a * 100  - d*10000 - e * 1000 - f * 100000) / 10;
        let c = num % 10;

        let numbers = [a, b, c, d, e, f];
        let mut even = 0;
        let mut odd = 0;
        for number in numbers.iter() {
            if number % 2 == 0{
                even += number;
            }else{
                odd += number;
            }
        }
        if even != 0{
            Some(format!("{}{}", even, odd))
        }else{
             Some(format!("{}{}", odd, even))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sum_even_odd_valid() {
        assert_eq!(sum_even_odd(111111), Some(format!("{}", 60)));   
        assert_eq!(sum_even_odd(123456), Some(format!("{}", 129)));   
        assert_eq!(sum_even_odd(999999), Some(format!("{}", 540)));  
        assert_eq!(sum_even_odd(100000), Some(format!("{}", 10)));   
    }

    #[test]
    fn test_sum_even_odd_invalid() {
        assert_eq!(sum_even_odd(99999), None);   // меньше 100000
        assert_eq!(sum_even_odd(1000000), None); // больше 999999
        assert_eq!(sum_even_odd(0), None);    // меньше 100000
    }
}