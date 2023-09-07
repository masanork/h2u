use std::fs::File;
use std::io::{self, BufRead, BufReader, Write};
use std::env;
use regex::Regex;

fn convert_hex_to_ucs(s: &str) -> String {
    // Variation Selectorを持つUCS文字の変換
    let re_vs = Regex::new(r"<U\+([0-9a-fA-F]{4,6}),U\+([0-9a-fA-F]{4,6})>").unwrap();
    let after_vs = re_vs.replace_all(s, |caps: &regex::Captures| {
        let base_hex = &caps[1];
        let selector_hex = &caps[2];
        match (
            u32::from_str_radix(base_hex, 16),
            u32::from_str_radix(selector_hex, 16),
        ) {
            (Ok(base_value), Ok(selector_value)) => match (
                char::from_u32(base_value),
                char::from_u32(selector_value),
            ) {
                (Some(base_char), Some(selector_char)) => {
                    format!("{}{}", base_char, selector_char)
                }
                _ => format!("<U+{},U+{}>", base_hex, selector_hex),
            },
            _ => format!("<U+{},U+{}>", base_hex, selector_hex),
        }
    });

    // 基本的なU+xxxx形式のコードの変換
    let re_basic = Regex::new(r"U\+([0-9a-fA-F]{4,6})").unwrap();
    let result = re_basic.replace_all(&after_vs, |caps: &regex::Captures| {
        let hex_value = &caps[1];
        match u32::from_str_radix(hex_value, 16) {
            Ok(value) => match char::from_u32(value) {
                Some(c) => c.to_string(),
                None => format!("U+{}", hex_value),
            },
            Err(_) => format!("U+{}", hex_value),
        }
    });
    result.to_string()
}

fn main() -> io::Result<()> {
    let file_name = env::args().nth(1).unwrap_or_else(|| {
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read from stdin");
        input.trim().to_string()
    });

    let file = File::open(file_name)?;
    let reader = BufReader::new(file);

    for line in reader.lines() {
        match line {
            Ok(l) => {
                let converted_line = convert_hex_to_ucs(&l);
                writeln!(io::stdout(), "{}", converted_line)?;
            },
            Err(e) => eprintln!("Error reading a line: {}", e),
        }
    }

    Ok(())
}
