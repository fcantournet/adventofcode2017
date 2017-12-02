use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();

    let input = &args[1];
    let out = captcha(input);
    println!("{}", out)
}


fn captcha(input: &String) -> u32 {
    let mut total = 0;
    let digits: Vec<_> = input.chars().map(|d| d.to_digit(10).unwrap()).collect();

    for i in 0..digits.len() {
        if digits[i] == digits[(i+1)%digits.len()] {
            total += digits[i]
        }
    }
    
    return total
}

#[test]
fn testcaptcha() {
    assert_eq!(captcha(&"91212129".to_string()),9);
    assert_eq!(captcha(&"1111".to_string()),4);
    assert_eq!(captcha(&"1234".to_string()),0);
    assert_eq!(captcha(&"1122".to_string()),3);
}