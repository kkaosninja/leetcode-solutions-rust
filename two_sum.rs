// Problem Page => https://leetcode.com/problems/two-sum/

impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
    
        if nums.len() == 2 {
            return vec![0,1];
        }

        for i in 0..nums.len()-1 {

            for j in i+1..nums.len() {

                if nums.get(i).unwrap() + nums.get(j).unwrap() == target {
                    return vec![i as i32, j as i32];
                }

            }

        }

        return vec![0,1];
    }
}
