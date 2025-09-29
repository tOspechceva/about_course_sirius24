// По данному числу N распечатайте все целые значения степени двойки, не превосходящие N, в порядке возрастания.
use super::common_functions;
use std::io;

pub fn task() -> io::Result<()> {
    let n = common_functions::input_int("число больше 1.")?;

    match no_more_than_power_two(n) {
        Some(res) => {
            println!("По числу {} = {:?}", n, res);
        }
        None => {
            println!("Число меньше 1");
        }
    }

    Ok(())
}

pub fn no_more_than_power_two(num: u32) -> Option<Vec<u32>> {
    if num < 1 {
        return None;
    }
    let mut vec = Vec::new();
    let mut power = 1u32; // 2^0 = 1

    while power <= num {
        vec.push(power);
        power *= 2;
    }

    Some(vec)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_no_more_than_power_two_valid() {
        assert_eq!(no_more_than_power_two(1), Some(vec![1]));
        assert_eq!(no_more_than_power_two(2), Some(vec![1, 2]));
        assert_eq!(no_more_than_power_two(4), Some(vec![1, 2, 4]));
        assert_eq!(no_more_than_power_two(6), Some(vec![1, 2, 4]));
        assert_eq!(no_more_than_power_two(9), Some(vec![1, 2, 4, 8]));
        assert_eq!(no_more_than_power_two(0), None);
    }
}