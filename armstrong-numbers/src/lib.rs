pub fn is_armstrong_number(num: u32) -> bool {
    let mut digits = 0;
    let mut number = num;
    while number > 0 {
        number = number.div_euclid(10);
        digits += 1;
    }

    println!("number {} has {} digits", num, digits);

    number = num;

    let mut sum = 0;
    while number > 0 {
        let digit = number.rem_euclid(10);
        println!("adding digit {} to the power of {}", digit, digits);
        sum += digit.pow(digits);
        number = number.div_euclid(10);
    }

    sum == num
}
