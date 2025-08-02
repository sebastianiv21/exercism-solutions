pub fn is_armstrong_number(num: u32) -> bool {
    let digits = num.to_string();
    let exp = digits.len() as u32;

    let sum = digits
        .chars()
        .fold(0, |acc, x| x.to_digit(10).unwrap().pow(exp) + acc);

    num == sum
}
