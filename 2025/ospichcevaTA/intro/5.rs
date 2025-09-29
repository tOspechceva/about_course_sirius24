//Вводится два целых числа a и b (a <= b). Найдите самое большее число на отрезке от a до b, кратное 7. Если такого числа нет выведите -1.
use super::common_functions;

use std::io;

pub fn task() -> io::Result<()> {
    let a = common_functions::input_int("число a.")?;
    let b = common_functions::input_int("число b.")?;

    match multiple_seven_segment(a, b) {
        Some(res) => {
            println!("Самое большее число на отрезке от {} до {}, кратное 7: {}", a, b, res);
        }
        None => {
            println!("Числа, кратного 7, на отрезке от {} до {} нет", a, b);
        }
    }

    Ok(())
}

pub fn multiple_seven_segment(a: u32, b: u32) -> Option<u32> {
    let (start, end) = if a <= b { (a, b) } else { (b, a) };

    let mut x = end;
    while x >= start {
        if x % 7 == 0 {
            return Some(x);
        }
        x -= 1;
    }

    None
}


#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_multiple_seven_segment_valid() {
        assert_eq!(multiple_seven_segment(1,8), Some(7));
        assert_eq!(multiple_seven_segment(12,15), Some(14));
        assert_eq!(multiple_seven_segment(1,25), Some(21));
        assert_eq!(multiple_seven_segment(4,6), None);
    }
}