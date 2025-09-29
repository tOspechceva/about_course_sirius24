//Дано натуральное число A > 1. Определите, каким по счету числом Фибоначчи оно является. Если А не является числом Фибоначчи, выведите число -1.
// Эта последовательность, например: 0, 1, 1, 2, 3, 5, 8, 13, 21, 34...
use super::common_functions;
use std::io;

pub fn task() -> io::Result<()> {
    let num = common_functions::input_int(" число.")?;

    let res = fibonacci_number(num);
    println!("Число является числом Фибоначчи под номером : {}", res.unwrap());

    Ok(())
}

pub fn fibonacci_number(num: u32) -> Option<i32> {
    if num < 1 {
        return Some(-1)
    }
    let mut x = 0;
    let mut y = 1;
    let mut count = 1;
    while y <= num{
        if y == num {break;}
        let res = x + y;
        x = y;
        y = res;
        count +=1;
    }

    if y == num{
        Some(count)
    }else{
        Some(-1)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_fibonacci_number_valid() {
        assert_eq!(fibonacci_number(1), Some(1));
        assert_eq!(fibonacci_number(2), Some(3));
        assert_eq!(fibonacci_number(3), Some(4));
        assert_eq!(fibonacci_number(4), Some(-1));
        assert_eq!(fibonacci_number(5), Some(5));
        assert_eq!(fibonacci_number(6), Some(-1));
        assert_eq!(fibonacci_number(7), Some(-1));
        assert_eq!(fibonacci_number(8), Some(6));
    }
}
