pub fn problem_136(nums: Vec<i32>) -> i32 {
    let mut ans: i32 = 0;
    for val in nums.iter() {
        ans = ans ^ *val;
    }
    return ans;
}
