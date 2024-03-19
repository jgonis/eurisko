pub fn binary_to_decimal(input: String) -> isize {
    0
}

pub fn hexadecimal_to_decimal(input: String) -> isize {
    0
}

pub fn decimal_to_binary(input: isize) -> String {
    String::from("0")
}

pub fn decimal_to_hexadecimal(input: isize) -> String {
    String::from("0")
}

pub fn binary_to_hexadecimal(input: String) -> String {
    String::from("0")
}

pub fn hexadecimal_to_binary(input: String) -> String {
    String::from("0")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_binary_to_decimal() {
        assert_eq!(binary_to_decimal(String::from("1010")), 10);
    }
}