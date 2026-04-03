pub fn rotate(nums: &mut Vec<i32>, k: i32)
{
    let k = (k as usize) % nums.len();
    
    // If rotating by k would result in no change, return early.
    if k == 0 { return; }
    
    reverse(nums, 0, nums.len());
    reverse(nums, 0, k);
    reverse(nums, k, nums.len());
}

fn reverse(nums: &mut Vec<i32>, start_inclusive: usize, end_exclusive: usize)
{
    for i in 0..(end_exclusive - start_inclusive) / 2
    {
        let left = start_inclusive + i;
        let right = end_exclusive - 1 - i;
        let temp = nums[left];
        nums[left] = nums[right];
        nums[right] = temp;
    }
}
