pub fn problem_136(nums: Vec<i32>) -> i32 {
    let mut ans: i32 = 0;
    for val in nums.iter() {
        ans = ans ^ *val;
    }
    return ans;
}
pub fn problem_509(n: i32) -> i32 {
    match n {
        0 | 1 => return n,
        _ => return problem_509(n - 1) + problem_509(n - 2),
    }
}
