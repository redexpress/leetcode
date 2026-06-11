use std::collections::HashMap;

// better than two_sum2
pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
    let n = nums.len();
    for i in 0..n {
        for j in (i+1)..n {
            if nums[i] + nums[j] == target {
                return vec![i as i32,j as i32];
            }
        }
    }
    vec![]
}

pub fn two_sum2(nums: Vec<i32>, target: i32) -> Vec<i32> {
    let mut map = HashMap::new();
    for (i, &num) in nums.iter().enumerate() {
        let complement = target - num;
        if let Some(&j) = map.get(&complement) {
            return vec![j as i32, i as i32];
        }
        map.insert(num, i as i32);
    }
    vec![]
}

#[cfg(test)]
mod tests {
    use super::*;

    fn sorted(mut v: Vec<i32>) -> Vec<i32> {
        v.sort();
        v
    }

    #[test]
    fn example() {
        assert_eq!(sorted(two_sum(vec![2, 7, 11, 15], 9)), vec![0, 1]);
    }

    #[test]
    fn middle_match() {
        assert_eq!(sorted(two_sum(vec![3, 2, 4], 6)), vec![1, 2]);
    }

    #[test]
    fn duplicate_values() {
        assert_eq!(sorted(two_sum(vec![3, 3], 6)), vec![0, 1]);
    }

    #[test]
    fn no_solution() {
        assert_eq!(two_sum(vec![1, 2, 3], 100), vec![]);
    }

    #[test]
    fn two_elements_match() {
        assert_eq!(sorted(two_sum(vec![1, 4], 5)), vec![0, 1]);
    }

    #[test]
    fn negative_numbers() {
        assert_eq!(sorted(two_sum(vec![-3, 4, 3, 90], 0)), vec![0, 2]);
    }

    #[test]
    fn hash_example() {
        assert_eq!(sorted(two_sum2(vec![2, 7, 11, 15], 9)), vec![0, 1]);
    }

    #[test]
    fn hash_middle_match() {
        assert_eq!(sorted(two_sum2(vec![3, 2, 4], 6)), vec![1, 2]);
    }

    #[test]
    fn hash_duplicate_values_returns_first_indices() {
        assert_eq!(two_sum2(vec![3, 3], 6), vec![0, 1]);
    }

    #[test]
    fn hash_no_solution() {
        assert_eq!(two_sum2(vec![1, 2, 3], 100), vec![]);
    }

    #[test]
    fn hash_two_elements_match() {
        assert_eq!(sorted(two_sum2(vec![1, 4], 5)), vec![0, 1]);
    }

    #[test]
    fn hash_negative_numbers() {
        assert_eq!(sorted(two_sum2(vec![-3, 4, 3, 90], 0)), vec![0, 2]);
    }

    #[test]
    fn hash_duplicate_value_later_index() {
        assert_eq!(two_sum2(vec![2, 7, 11, 15, 2], 4), vec![0, 4]);
    }

    #[test]
    fn hash_single_element() {
        assert_eq!(two_sum2(vec![5], 10), vec![]);
    }

    #[test]
    fn both_agree_random() {
        let cases: Vec<(Vec<i32>, i32)> = vec![
            (vec![1, 5, 3, 7], 8),
            (vec![0, 0, 0, 0], 0),
            (vec![-1, -2, -3, -4], -7),
            (vec![10, -10, 20, -20], 0),
            (vec![100, 200, 300, 400, 500], 900),
        ];
        for (nums, target) in cases {
            let a = two_sum(nums.clone(), target);
            let b = two_sum2(nums.clone(), target);
            assert!(!a.is_empty() && !b.is_empty(), "expected a solution for {nums:?} target={target}");
            let (i, j) = (a[0] as usize, a[1] as usize);
            assert_eq!(nums[i] + nums[j], target);
        }
    }
}