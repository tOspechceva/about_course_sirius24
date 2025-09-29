//На ввод подаются N целых чисел, их нужно сохранить в массив или список. Затем вывести макимальный элемент.
use super::common_functions;
use std::io;

pub fn task() -> io::Result<()> {
    let n: u32 = common_functions::input_int("Введите количество чисел (N): ")?;

    if n == 0 {
        println!("Количество чисел должно быть больше 0.");
        return Ok(());
    }

    let mut vec = Vec::new();
    for _ in 0..n {
        let num = common_functions::input_int("Введите число: ")?;
        vec.push(num);
    }

    match max(&vec) {
        Some(res) => {
            println!("Максимальный элемент: {}", res);
        }
        None => {
            println!("Список пуст");
        }
    }

    Ok(())
}

pub fn max(numbers: &[u32]) -> Option<u32> {
    if numbers.is_empty() {
        return None;
    }

    let mut max_val = numbers[0];
    for &num in numbers.iter().skip(1) {
        if num > max_val {
            max_val = num;
        }
    }
    Some(max_val)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_max() {
        assert_eq!(max(&[]), None);
        assert_eq!(max(&[5]), Some(5));
        assert_eq!(max(&[1, 2, 3]), Some(3));
        assert_eq!(max(&[10, 3, 7, 2]), Some(10));
        assert_eq!(max(&[0, 0, 0]), Some(0));
    }
}