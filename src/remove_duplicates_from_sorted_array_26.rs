pub fn remove_duplicates(nums: &mut Vec<i32>) -> i32
{
    if nums.is_empty()
    {
        return 0;
    }

    let mut current_value_idx = 0;
    let mut next_unique_value_index = 1;
    let mut counter = 1;

    while next_unique_value_index < nums.len()
    {
        // If we have a new unique value
        if nums[next_unique_value_index] > nums[current_value_idx]
        {
            counter += 1;
            current_value_idx += 1;
            let temp = nums[current_value_idx];
            nums[current_value_idx] = nums[next_unique_value_index];
            nums[next_unique_value_index] = temp;
        }

        next_unique_value_index += 1;
    }

    counter
}
