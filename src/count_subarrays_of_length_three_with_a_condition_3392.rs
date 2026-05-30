pub fn count_subarrays(nums: Vec<i32>) -> i32
{
    nums.windows(3)
        .filter(|subarray| (subarray[0] + subarray[2]) * 2 == subarray[1])
        .count() as i32
}
