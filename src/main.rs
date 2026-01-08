use rand::random_range;
use std::io;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut max_l = String::new();

    let mut uppercase = String::new();

    let mut lowercase = String::new();

    let mut numbers = String::new();

    let mut strange_symbols = String::new();

    println!("\x1b[1;31mПриветствую в генераторе паролей!\x1b[0m");

    println!("\x1b[31mКакой длины хотите пароль?\x1b[0m");

    io::stdin().read_line(&mut max_l)?;

    println!(
        "\x1b[41mВводите либо 1(да), либо 0(нет). Если введёте любое другое число/символ это будет расцениваться как ДА\x1b[0m"
    );

    println!("\x1b[4;32mПароль должен содержать заглавные буквы?\x1b[0m");

    io::stdin().read_line(&mut uppercase)?;

    println!("\x1b[4;32mПароль должен содержать маленькие буквы?\x1b[0m");

    io::stdin().read_line(&mut lowercase)?;

    println!("\x1b[4;32m должен содержать цифры?\x1b[0m");

    io::stdin().read_line(&mut numbers)?;

    println!("\x1b[4;32mПароль должен содержать странные символы(_=! и т.д.)?\x1b[0m");

    io::stdin().read_line(&mut strange_symbols)?;

    let uppercase = uppercase.trim().parse::<i32>()? != 0;

    let lowercase = lowercase.trim().parse::<i32>()? != 0;

    let numbers = numbers.trim().parse::<i32>()? != 0;

    let strange_symbols = strange_symbols.trim().parse::<i32>()? != 0;

    let mut i = 0;

    println!("\x1b[1;33mСколько хотите паролей получить?\x1b[0m");

    let mut passwords = String::new();

    io::stdin().read_line(&mut passwords)?;

    println!("\x1b[48;2;0;100;255mПароли:\x1b[0m");

    while i < passwords.trim().parse::<i32>()? {
        let res = generate_password(
            max_l.trim().parse::<i32>()?,
            uppercase,
            lowercase,
            numbers,
            strange_symbols,
        );

        println!("{res}");

        i += 1;
    }
    println!("\n\x1b[1;33mПрограмма завершена. Нажмите Enter, чтобы закрыть окно...\x1b[0m");
    let mut wait = String::new();
    let _ = io::stdin().read_line(&mut wait);
    Ok(())
}

fn generate_password(
    max_l: i32,
    uppercase: bool,
    lowercase: bool,
    numbers: bool,
    strange_symbols: bool,
) -> String {
    const CHAR_UPPER_CASE: &[u8] = b"QWERTYUIOPASDFGHJKLZXCVBNM";

    const CHAR_LOWER_CASE: &[u8] = b"qwertyuiopasdfghjklzxcvbnm";

    const CHAR_NUMBERS: &[u8] = b"0123456789";

    const CHAR_STRANGE_SYMB: &[u8] = b"~!@#$%^&*()_=+`|{}[];'<>,.?-";

    let mut char_set = Vec::<u8>::new();

    if uppercase {
        char_set.extend_from_slice(CHAR_UPPER_CASE);
    }
    if lowercase {
        char_set.extend_from_slice(CHAR_LOWER_CASE);
    }
    if numbers {
        char_set.extend_from_slice(CHAR_NUMBERS);
    }
    if strange_symbols {
        char_set.extend_from_slice(CHAR_STRANGE_SYMB);
    }

    if char_set.is_empty() {
        return String::new();
    }
    let mut final_str = String::new();
    let mut i = 0;
    while i < max_l {
        let rand_i = random_range(0..char_set.len());
        let j = char_set[rand_i] as char;
        final_str = [final_str, j.to_string()].concat();
        i += 1;
    }

    final_str
}
