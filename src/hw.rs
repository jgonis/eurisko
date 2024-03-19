use std::collections::HashMap;

pub fn check_if_symmetric(str: String) -> bool {
    let chars = str.chars().collect::<Vec<char>>();
    for i in 0..chars.len() / 2 {
        if chars[i].to_ascii_lowercase() != chars[chars.len() - i - 1].to_ascii_lowercase() {
            return false;
        }
    }
    true
}

pub fn convert_to_numbers(str: String) -> Vec<usize> {
    let mut result = Vec::new();
    for x in str.chars() {
        result.push(char_to_number(x));
    }
    result
}

fn char_to_number(c: char) -> usize {
    match c {
        ' ' => 0,
        'a' => 1,
        'b' => 2,
        'c' => 3,
        'd' => 4,
        'e' => 5,
        'f' => 6,
        'g' => 7,
        'h' => 8,
        'i' => 9,
        'j' => 10,
        'k' => 11,
        'l' => 12,
        'm' => 13,
        'n' => 14,
        'o' => 15,
        'p' => 16,
        'q' => 17,
        'r' => 18,
        's' => 19,
        't' => 20,
        'u' => 21,
        'v' => 22,
        'w' => 23,
        'x' => 24,
        'y' => 25,
        'z' => 26,
        _ => panic!("Invalid character"),
    }
}

fn number_to_char(i: usize) -> char {
    match i {
        0 => ' ',
        1 => 'a',
        2 => 'b',
        3 => 'c',
        4 => 'd',
        5 => 'e',
        6 => 'f',
        7 => 'g',
        8 => 'h',
        9 => 'i',
        10 => 'j',
        11 => 'k',
        12 => 'l',
        13 => 'm',
        14 => 'n',
        15 => 'o',
        16 => 'p',
        17 => 'q',
        18 => 'r',
        19 => 's',
        20 => 't',
        21 => 'u',
        22 => 'v',
        23 => 'w',
        24 => 'x',
        25 => 'y',
        26 => 'z',
        _ => panic!("Invalid number"),
    }
}

pub fn convert_to_letters(input: &[usize]) -> String {
    let mut result = String::new();
    for x in input {
        result.push(number_to_char(*x));
    }
    result
}

pub fn get_intersection(input1: &[isize], input2: &[isize]) -> Vec<isize> {
    let mut result = Vec::new();
    for x in input1 {
        if is_member(*x, input2) && !is_member(*x, &result) {
            result.push(*x);
        }
    }
    result
}

fn is_member(input: isize, list: &[isize]) -> bool {
    for x in list {
        if x == &input {
            return true;
        }
    }
    false
}

pub fn get_union(input1: &[isize], input2: &[isize]) -> Vec<isize> {
    let mut result = Vec::new();
    for x in input1 {
        if !is_member(*x, &result) {
            result.push(*x);
        }
    }
    for x in input2 {
        if !is_member(*x, &result) {
            result.push(*x);
        }
    }
    result
}

pub fn count_characters(str: String) -> HashMap<char, isize> {
    let mut result = HashMap::new();
    for x in str.chars() {
        let entry_value = result.entry(x.to_ascii_lowercase()).or_insert(0);
        *entry_value += 1;
    }
    result
}

pub fn is_prime(num: isize) -> bool {
    let limit = (num as f64).sqrt() as isize;
    if num == 1 {
        return false;
    }
    if num % 2 == 0 {
        return false;
    }

    for i in 3..limit {
        if num % i == 0 {
            return false;
        }
    }
    true
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_check_if_empty_string_symmetric() {
        assert!(check_if_symmetric("".to_string()));
        assert!(check_if_symmetric("a".to_string()));
        assert!(check_if_symmetric("aa".to_string()));
        assert!(check_if_symmetric("aba".to_string()));
        assert!(check_if_symmetric("racecar".to_string()));
        assert!(check_if_symmetric("racEcaR".to_string()));
        assert!(check_if_symmetric("!ab123 4 321ba!".to_string()));

        assert!(!check_if_symmetric("racecars".to_string()));
        assert!(!check_if_symmetric("ab".to_string()));
    }

    #[test]
    fn test_convert_to_numbers() {
        assert_eq!(convert_to_numbers("a cat".to_string()), [1, 0, 3, 1, 20]);
    }

    #[test]
    fn test_convert_to_letters() {
        assert_eq!(convert_to_letters(&[1, 0, 3, 1, 20]), "a cat".to_string());
    }

    #[test]
    fn test_get_intersection() {
        assert_eq!(get_intersection(&[1, 0, 3, 1, 20], &[1, 0, 3, 1, 20]), [1, 0, 3, 20]);
        assert_eq!(get_intersection(&[], &[1, 0, 3, 1, 20]), []);
        assert_eq!(get_intersection(&[1, 0, 3, 1, 20], &[]), []);
        assert_eq!(get_intersection(&[1, 0, 3, 1, 20], &[2, 4, 6, 8]), []);
        assert_eq!(get_intersection(&[1, 1, 1, 1, 1], &[1, 1, 1, 1]), [1]);
    }

    #[test]
    fn test_get_union() {
        assert_eq!(get_union(&[1, 0, 3, 1, 20], &[1, 0, 3, 1, 20]), [1, 0, 3, 20]);
        assert_eq!(get_union(&[], &[1, 0, 3, 1, 20]), &[1, 0, 3, 20]);
        assert_eq!(get_union(&[1, 0, 3, 1, 20], &[]), &[1, 0, 3, 20]);
        assert_eq!(get_union(&[1, 0, 3, 1, 20], &[2, 4, 6, 8]), &[1, 0, 3, 20, 2, 4, 6, 8]);
        assert_eq!(get_union(&[1, 1, 1, 1, 1], &[1, 1, 1, 1]), [1]);
    }

    #[test]
    fn test_count_characters() {
        assert_eq!(count_characters("A cat!!!".to_string()), HashMap::from([('a', 2), ('c', 1), ('t', 1), (' ', 1), ('!', 3)]));
        assert_eq!(count_characters("".to_string()), HashMap::new());
    }

    #[test]
    fn test_is_prime() {
        assert!(!is_prime(1));
        assert!(!is_prime(2));
        assert!(is_prime(3));
        assert!(is_prime(5));
        assert!(is_prime(7));
        assert!(!is_prime(6));
    }

    // 2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37, 41, 43, 47, 53, 59, 61, 67, 71, 73, 79, 83, 89, 97
}