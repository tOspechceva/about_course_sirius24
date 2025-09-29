//Даны два числа. Определить цифры, входящие в запись как первого, так и второго числа. Программа должна вывести цифры, которые имеются в обоих числах, через пробел. Цифры выводятся в порядке их нахождения в первом числе.
use super::common_functions;
use std::io;
use std::collections::HashSet;

pub fn task() -> io::Result<()> {
    let num1: i64 = common_functions::input_int("Введите первое число: ")?;
    let num2: i64 = common_functions::input_int("Введите второе число: ")?;

    let common_digits = find_common_digits(num1, num2);
    
    if common_digits.is_empty() {
        println!("Общих цифр нет.");
    } else {
        println!("Общие цифры: {}", common_digits.join(" "));
    }

    Ok(())
}

pub fn find_common_digits(a: i64, b: i64) -> Vec<String> {
    let digits_b: HashSet<char> = b.abs().to_string().chars().collect();
    
    let digits_a_str = a.abs().to_string();
    
    let mut result = Vec::new();
    let mut seen = HashSet::new(); 

    for ch in digits_a_str.chars() {
        if digits_b.contains(&ch) && !seen.contains(&ch) {
            result.push(ch.to_string());
            seen.insert(ch);
        }
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_find_common_digits() {
        assert_eq!(find_common_digits(123, 321), vec!["1", "2", "3"]);
        assert_eq!(find_common_digits(505, 102), vec!["5", "0"]);
        assert_eq!(find_common_digits(9876, 1234), Vec::<String>::new());
        assert_eq!(find_common_digits(112233, 231), vec!["1", "2", "3"]);
        assert_eq!(find_common_digits(-123, 3210), vec!["1", "2", "3"]);
        assert_eq!(find_common_digits(0, 0), vec!["0"]);
        assert_eq!(find_common_digits(100, 200), vec!["0"]);
    }
}