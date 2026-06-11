
pub fn length_of_longest_substring(s: String) -> i32 {
    let mut set = std::collections::HashSet::new();
    let mut left = 0;
    let mut result = 0;
    let chars: Vec<char> = s.chars().collect();
    for right in left..s.len() {
        while set.contains(&chars[right]) {
            set.remove(&chars[left]);
            left += 1;
        }
        set.insert(chars[right]);
        result = result.max(right - left + 1);
    }
    result as i32
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_abcabcbb() {
        assert_eq!(length_of_longest_substring("abcabcbb".to_string()), 3);
    }

    #[test]
    fn example_bbbbb() {
        assert_eq!(length_of_longest_substring("bbbbb".to_string()), 1);
    }

    #[test]
    fn example_pwwkew() {
        assert_eq!(length_of_longest_substring("pwwkew".to_string()), 3);
    }
}

