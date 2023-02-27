// TODO: remove this when you're done with your implementation.
#![allow(unused_variables, dead_code)]

pub fn luhn(cc_number: &str) -> bool {
    let mut space_ignored: String = cc_number.to_string();
    space_ignored.retain(|c| c.is_numeric());

    let mut sum = 0;
    if space_ignored.len() < 2 {
        return false;
    }

    for i in 0..(space_ignored.len()) {
        let focusing_character = space_ignored.chars().nth(space_ignored.len() - i -1).unwrap();
        if i % 2 == 0 {
            sum += focusing_character.to_digit(10).unwrap();
        } else {
            let tmp = focusing_character.to_digit(10).unwrap() * 2;
            let tmp_str = tmp.to_string();
            let mut doubled_sum = 0;
            for j in 0..(tmp_str.len()) {
                doubled_sum += tmp_str.chars().nth(j).unwrap().to_digit(10).unwrap();
            }
            sum += doubled_sum;
        }
    }
    return sum % 10 == 0;
}

#[test]
fn test_non_digit_cc_number() {
    assert!(!luhn("foo"));
}

#[test]
fn test_empty_cc_number() {
    assert!(!luhn(""));
    assert!(!luhn(" "));
    assert!(!luhn("  "));
    assert!(!luhn("    "));
}

#[test]
fn test_single_digit_cc_number() {
    assert!(!luhn("0"));
}

#[test]
fn test_two_digit_cc_number() {
    assert!(luhn(" 0 0 "));
}

#[test]
fn test_valid_cc_number() {
    assert!(luhn("4263 9826 4026 9299"));
    assert!(luhn("4539 3195 0343 6467"));
    assert!(luhn("7992 7398 713"));
}

#[test]
fn test_invalid_cc_number() {
    assert!(!luhn("4223 9826 4026 9299"));
    assert!(!luhn("4539 3195 0343 6476"));
    assert!(!luhn("8273 1232 7352 0569"));
}

#[allow(dead_code)]
fn main() {
    luhn("foo");
}
