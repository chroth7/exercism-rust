fn get_digits(mut num: u32) -> Vec<u32> {
    let mut digits: Vec<u32> = vec![];
    while num > 0 {
        digits.push(num % 10);
        num /= 10;
    }
    digits
}

pub fn is_armstrong_number(num: u32) -> bool {
    let digits = get_digits(num);
    num == digits
        .iter()
        .fold(0, |acc, d| acc + d.pow(digits.len() as u32))
}
