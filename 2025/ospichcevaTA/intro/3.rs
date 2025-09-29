//Вводится натуральное число N, а затем N чисел. По данным числам, определите количество чисел, которые равны нулю.
use super::common_functions;
use std::io;

pub fn task() -> io::Result<()> {
    let count = common_functions::input_int(" число, кол-во элементов.")?;
    
    let mut numbers = vec![];
    for _ in 0..count{ 
        numbers.push(common_functions::input_int(" число")?);
    }

    let res = quantity_zero(numbers);
    println!("Кол-во чисел равных 0 : {}", res.unwrap());
    
    Ok(())
}

pub fn quantity_zero(numbers: Vec<u32>) -> Option<u32> {
    let mut res = 0;
    for number in numbers.iter() {
        if *number == 0{
            res += 1;
        }
    }
    Some(res)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_quantity_zero_valid() {
        assert_eq!(quantity_zero(vec![0,1]), Some(1));   
        assert_eq!(quantity_zero(vec![1,2,3,4,5,6]), Some(0));   
        assert_eq!(quantity_zero(vec![1,2,0,4,5,0]), Some(2));  
        assert_eq!(quantity_zero(vec![0,0,0,0,0,0]), Some(6));   
    }
}