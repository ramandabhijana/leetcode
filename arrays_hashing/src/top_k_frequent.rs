/// Top K Frequent Elements
/// Given an integer array nums and an integer k, return the k most frequent elements. You may return the answer in any order
///
/// Example 1:
///     Input: nums = [1,1,1,2,2,3], k = 2
///     Output: [1,2]
///
/// Example 2:
///     Input: nums = [1], k = 1
///     Output: [1]
///
/// Constraints:
///     1 <= nums.length <= 105
///     -104 <= nums[i] <= 104
///     k is in the range [1, the number of unique elements in the array].
///     It is guaranteed that the answer is unique.
///

struct Solution;
impl Solution {
    pub fn top_k_frequent(nums: Vec<i32>, k: i32) -> Vec<i32> {
        use std::collections::HashMap;

        if nums.len() == k as usize {
            return nums;
        }

        let mut nums_count: HashMap<i32, i32> = HashMap::new();

        for i in nums {
            nums_count.entry(i).and_modify(|n| *n += 1).or_insert(1);
        }

        let mut sorted_by_occurence = Vec::from_iter(nums_count);
        sorted_by_occurence.sort_by(|&lhs, &rhs| rhs.1.cmp(&lhs.1));

        sorted_by_occurence
            .into_iter()
            .take(k as usize)
            .map(|(k, _)| k)
            .collect()
    }
}

#[cfg(test)]
mod test {
    use super::Solution;

    #[test]
    fn test_case_1() {
        let nums: Vec<i32> = [1, 1, 1, 2, 2, 3].into();
        let target = 2i32;
        assert_eq!(Solution::top_k_frequent(nums, target), [1, 2].to_vec());
    }

    #[test]
    fn test_case_2() {
        let nums: Vec<i32> = [1].into();
        let target = 1i32;
        assert_eq!(Solution::top_k_frequent(nums, target), [1].to_vec());
    }
}
