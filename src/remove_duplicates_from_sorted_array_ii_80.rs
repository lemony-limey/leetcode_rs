pub fn remove_duplicates(nums: &mut Vec<i32>) -> i32
{
    if nums.is_empty()
    {
        return 0;
    }

    let mut current = 0;
    let mut next_unique = 1;
    let mut unique_elements = 1;
    let mut duplicate_counter = 1;

    while next_unique < nums.len()
    {
        // This is the first time we are encountering the new element.
        if nums[next_unique] > nums[current]
        {
            unique_elements += 1;
            current += 1;
            duplicate_counter = 1;

            let temp = nums[current];
            nums[current] = nums[next_unique];
            nums[next_unique] = temp;
        }
        // This is the second time we are encountering the new element.
        else if duplicate_counter == 1 && nums[next_unique] == nums[current]
        {
            unique_elements += 1;
            current += 1;
            duplicate_counter += 1;

            let temp = nums[current];
            nums[current] = nums[next_unique];
            nums[next_unique] = temp;
        }

        next_unique += 1;
        duplicate_counter = duplicate_counter % 2;
    }

    unique_elements
}
