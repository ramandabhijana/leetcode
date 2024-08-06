struct Solution;
impl Solution {
    pub fn contains_duplicate(nums: Vec<i32>) -> bool {
        use std::collections::HashSet;

        let mut set: HashSet<i32, _> = HashSet::new();

        for num in nums {
            if set.contains(&num) {
                return true;
            }
            set.insert(num);
        }

        false
    }
}

#[cfg(test)]
mod test {
    use super::Solution;

    #[test]
    fn test_case_1() {
        let nums: Vec<i32> = [1, 2, 3, 1].into();
        assert!(Solution::contains_duplicate(nums));
    }

    #[test]
    fn test_case_2() {
        let nums: Vec<i32> = [1, 2, 3, 4].into();
        assert!(!Solution::contains_duplicate(nums));
    }

    #[test]
    fn test_case_3() {
        let nums: Vec<i32> = [1, 1, 1, 3, 3, 4, 3, 2, 4, 2].into();
        assert!(Solution::contains_duplicate(nums));
    }
}
