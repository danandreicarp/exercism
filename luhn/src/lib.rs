/// Check a Luhn checksum.
pub fn is_valid(code: &str) -> bool {
    let code = code.replace(' ', "");

    if code.len() < 2 {
        false
    } else {
        let mut sum: u8 = 0;
        let mut should_double = true;
        for ch in code.chars().rev() {
            should_double = !should_double;
            match (ch.to_digit(10), should_double) {
                (Some(d), true) => {
                    let mut dbl = d as u8 * 2;
                    if dbl > 9 {
                        dbl -= 9;
                    }
                    sum += dbl;
                }
                (Some(d), false) => {
                    sum += d as u8;
                }
                _ => {
                    return false;
                }
            }
        }

        sum % 10 == 0
    }
}
