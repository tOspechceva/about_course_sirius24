/* В цепочках ДНК символы «А» и «Т» дополняют друг друга, как «С» и «G». Нужно написать программу, которая получате на вход последовательность ДНК и на выходе отображает комлементарную ей.
 */
use super::common_functions;
use std::io;

pub fn task() -> io::Result<()> {
    let dna_str = common_functions::input_str("строку ДНК")?;

    match dna_census(dna_str) {
        Some(complement) => println!("Комплементарная последовательность: {}", complement),
        None => println!("Ошибка: строка содержит недопустимые символы"),
    }

    Ok(())
}

pub fn dna_census(a: String) -> Option<String> {
    let mut result = String::new();

    for c in a.chars() {
        match c {
            'A' => result.push('T'),
            'T' => result.push('A'),
            'C' => result.push('G'),
            'G' => result.push('C'),
            _ => return None, // недопустимый символ
        }
    }

    Some(result)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_dna_census_valid() {
        assert_eq!(dna_census(String::from("GTAT")), Some(String::from("CATA")));
        assert_eq!(dna_census(String::from("")), Some(String::from("")));
        assert_eq!(dna_census(String::from("ATCG")), Some(String::from("TAGC")));
        assert_eq!(dna_census(String::from("AAAA")), Some(String::from("TTTT")));
    }

    #[test]
    fn test_dna_census_invalid() {
        assert_eq!(dna_census(String::from("AXT")), None);
        assert_eq!(dna_census(String::from("atcg")), None); // строчные — недопустимы
        assert_eq!(dna_census(String::from("GCT!")), None);
    }
}
