pub fn search(nums: Vec<i32>, target: i32) -> bool
{
    let (mut left, mut right) = (0, nums.len() - 1);

    // less than OR EQUAL TO because array could be [1].
    let mut middle;
    while left <= right
    {
        middle = (left + right) / 2;

        // Return the index if we've found the target value.
        if target == nums[middle] { return true; }

        // Check which sorted portion we're in.
        if nums[left] < nums[middle] // Left sorted portion.
        {
            if nums[left] <= target && target < nums[middle]
            {
                // Continue searching in the left portion.
                right = middle - 1;
            }
            else
            {
                left = middle + 1;
            }
        }
        else if nums[left] > nums[middle] // Right sorted portion.
        {
            // Check if we need to search the left portion.
            if nums[middle] < target && target <= nums[right]
            {
                left = middle + 1;
            }
            else
            {
                right = middle - 1;
            }
        }
        // Left value is equal to the middle, so we cannot yet determine which side the middle
        // pointer is in.
        else
        {
            left += 1;
        }
    }

    false
}
