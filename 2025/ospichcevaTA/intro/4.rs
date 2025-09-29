//Вводится натуральное число N, а затем N целых чисел последовательности. Найдите количество минимальных элементов в последовательности.
use super::common_functions;
use std::io;

pub fn task() -> io::Result<()> {
    let count = common_functions::input_int(" число, кол-во элементов.")?;

    let mut numbers = vec![];
    for _ in 0..count {
        numbers.push(common_functions::input_int(" число")?);
    }

    let res = number_min_elements(numbers);
    println!("Кол-во минимальных чисел : {}", res.unwrap());

    Ok(())
}

pub fn number_min_elements(numbers: Vec<u32>) -> Option<u32> {
    if numbers.is_empty() {
        return Some(0);
    }

    let mut count = 0;
    let mut min = numbers[0];
    for number in numbers.iter() {
        if *number == min {
            count += 1;
        }
        if *number < min {
            count = 1;
            min = *number;
        }
    }

    Some(count)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_number_min_elements_valid() {
        assert_eq!(number_min_elements(vec![0, 1]), Some(1));
        assert_eq!(number_min_elements(vec![1, 2, 3, 4, 5, 6]), Some(1));
        assert_eq!(number_min_elements(vec![1, 2, 0, 4, 5, 0]), Some(2));
        assert_eq!(number_min_elements(vec![0, 0, 0, 0, 0, 0]), Some(6));
    }
}
