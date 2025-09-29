use std::io;

pub fn input_str(prompt: &str) -> io::Result<String> {
    let mut input = String::new();
    println!("Введите: {}", prompt);
    io::stdin().read_line(&mut input)?;
    Ok(input.trim().to_string()) 
}

pub fn input_int(prompt: &str) -> io::Result<u32> {
    let input = input_str(prompt)?;
    let num: u32 = input
        .parse()
        .map_err(|_| io::Error::new(io::ErrorKind::InvalidInput, "Не число"))?;
    Ok(num) 
}