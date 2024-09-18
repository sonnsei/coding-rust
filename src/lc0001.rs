use std::collections::HashMap;

impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let mut map = HashMap::new();
        for (i, v) in nums.iter().enumerate(){
            if let Some(&x) = map.get(&(target - v)){
                return vec![x, i as i32]
            }
            map.insert(v, i as i32);
        }
        vec![]
    }
}