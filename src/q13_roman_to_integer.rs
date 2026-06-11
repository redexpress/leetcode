pub fn roman_to_int(s: String) -> i32 {
    let mut result = 0;
    let mut pre_value = 0;
    for c in s.chars() {
        let value = match c {
            'I' => 1,
            'V' => 5,
            'X' => 10,
            'L' => 50,
            'C' => 100,
            'D' => 500,
            'M' => 1000,
            _ => 0,
        };
        if pre_value < value {
            result -= pre_value
        } else {
            result += pre_value;
        }
        pre_value = value;
    }
    result + pre_value
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_iii() {
        assert_eq!(roman_to_int(String::from("III")), 3);
    }

    #[test]
    fn example_lviii() {
        assert_eq!(roman_to_int(String::from("LVIII")), 58);
    }

    #[test]
    fn example_mcmxciv() {
        assert_eq!(roman_to_int(String::from("MCMXCIV")), 1994);
    }
}