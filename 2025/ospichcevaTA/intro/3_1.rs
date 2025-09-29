//Вводится натуральное число N, а затем N чисел. Найти среднее арифметическое всех чисел кратных 3. Если таких чисел нет, то вывести -1
use super::common_functions;
use std::io;

pub fn task() -> io::Result<()> {
    let count = common_functions::input_int(" число, кол-во элементов.")?;
    
    let mut numbers = vec![];
    for _ in 0..count{ 
        numbers.push(common_functions::input_int(" число")?);
    }

    let res = average_arithmetic_multiple_3(numbers);
    println!("Cреднее арифметическое всех чисел кратных 3 : {}", res.unwrap());
    
    Ok(())
}

pub fn average_arithmetic_multiple_3(numbers: Vec<u32>) -> Option<f32> {
    let mut count = 0;
    let mut sum = 0;
    for number in numbers.iter() {
        if *number % 3 == 0 && *number != 0{
            count += 1;
            sum += *number;
        }
    }
    if count == 0 {
        Some((-1) as f32)
    }else{
        Some(sum as f32 / count as f32)
    }

    
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_average_arithmetic_multiple_3_valid() {
        assert_eq!(average_arithmetic_multiple_3(vec![0,1]), Some(-1.0));   
        assert_eq!(average_arithmetic_multiple_3(vec![1,2,3,4,5,6]), Some(4.5));   
        //assert_eq!(average_arithmetic_multiple_3(vec![1,2,0,4,5,6]), Some(6.0));  
    }
}