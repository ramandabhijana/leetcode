/// Product of Array Except Self
/// Given an integer array nums, return an array answer such that answer[i] is equal to the product of all the elements of nums except nums[i].
///
/// The product of any prefix or suffix of nums is guaranteed to fit in a 32-bit integer.
///
/// You must write an algorithm that runs in O(n) time and without using the division operation.
///
/// Example 1:
///     Input: nums = [1,2,3,4]
///     Output: [24,12,8,6]
///
/// Example 2:
///     Input: nums = [-1,1,0,-3,3]
///     Output: [0,0,9,0,0]
///
/// Constraints:
///     2 <= nums.length <= 105
///     -30 <= nums[i] <= 30
///     The product of any prefix or suffix of nums is guaranteed to fit in a 32-bit integer.
///

struct Solution;
impl Solution {
    pub fn product_except_self(nums: Vec<i32>) -> Vec<i32> {
        let mut result: Vec<i32> = Vec::new();

        let mut prefix = 1i32;
        for i in 0..nums.len() {
            result.push(prefix);
            prefix *= nums[i];
        }

        let mut postfix = 1i32;
        for i in (0..nums.len()).rev() {
            result[i] *= postfix;
            postfix *= nums[i];
        }

        result
    }
}

#[cfg(test)]
mod test {
    use super::Solution;

    #[test]
    fn test_case_1() {
        let nums: Vec<i32> = [1, 2, 3, 4].into();
        assert_eq!(Solution::product_except_self(nums), [24, 12, 8, 6].to_vec());
    }

    #[test]
    fn test_case_2() {
        let nums: Vec<i32> = [-1, 1, 0, -3, 3].into();
        assert_eq!(
            Solution::product_except_self(nums),
            [0, 0, 9, 0, 0].to_vec()
        );
    }
}
