pub fn search(nums: Vec<i32>, target: i32) -> i32
{
    let (mut left, mut right) = (0, nums.len() - 1);

    // less than OR EQUAL TO because array could be [1].
    let mut middle;
    while left <= right
    {
        middle = (left + right) / 2;

        // Return the index if we've found the target value.
        if target == nums[middle] { return middle as i32; }

        // Check which sorted portion we're in.
        // Left portion (<= because it's possible left and middle could be the same value).
        if nums[left] <= nums[middle]
        {
            // Check if we need to search the right portion.
            if target > nums[middle] || target < nums[left]
            {
                left = middle + 1;
            }
            else
            {
                right = middle - 1;
            }
        }
        else // Right sorted portion.
        {
            // Check if we need to search the left portion.
            if target < nums[middle] || target > nums[right]
            {
                right = middle - 1;
            }
            else
            {
                 left = middle + 1;
            }
        }
    }

    -1
}
