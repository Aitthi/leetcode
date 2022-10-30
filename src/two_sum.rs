
pub struct Solution;

// Given an array of integers nums and an integer target, return indices of the two numbers such that they add up to target.
// You may assume that each input would have exactly one solution, and you may not use the same element twice.    
// You can return the answer in any order.
impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let mut map: std::collections::BinaryHeap<(_, &_)> = nums.iter().enumerate().collect();
        while let Some((i, &num)) = map.pop() {
            if let Some((j, &_)) = map.iter().find(|(_, &num2)| num2 == target - num) {
                return vec![*j as i32, i as i32];
            }
        }
        vec![]
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    // [2, 7, 11, 15]
    // 9
    #[test]
    fn target_9() {
        let nums = vec![2, 7, 11, 15];
        let target = 9;
        let result = Solution::two_sum(nums, target);
        assert_eq!(result, vec![0, 1]);
    }
    
    // [3,2,4]
    // 6
    #[test]
    fn target_6() {
        let nums = vec![3, 2, 4];
        let target = 6;
        let result = Solution::two_sum(nums, target);
        assert_eq!(result, vec![1, 2]);
    }

    // [3,3]
    // 6
    #[test]
    fn target_6_2() {
        let nums = vec![3, 3];
        let target = 6;
        let result = Solution::two_sum(nums, target);
        assert_eq!(result, vec![0, 1]);
    }

    // [0,4,3,0]
    // 0
    #[test]
    fn target_0() {
        let nums = vec![0, 4, 3, 0];
        let target = 0;
        let result = Solution::two_sum(nums, target);
        assert_eq!(result, vec![0, 3]);
    }

    // [-3,4,3,90]
    // 0
    #[test]
    fn target_0_2() {
        let nums = vec![-3, 4, 3, 90];
        let target = 0;
        let result = Solution::two_sum(nums, target);
        assert_eq!(result, vec![0, 2]);
    }

}
