use std::collections::HashMap;

pub fn sum_of_unique(nums: Vec<i32>) -> i32
{
    let mut unique: HashMap<i32, bool> = HashMap::new();

    // Add all values to a HashMap and keep track of which elements
    // are unique.
    for num in nums
    {
        if let Some(value) = unique.get(&num)
        {
            if *value
            {
                unique.insert(num, false);
            }
        }
        else
        {
            unique.insert(num, true);
        }
    }

    // Count the number of unique values
    unique
        .iter()
        .filter(|(_key, value)| **value)
        .fold(0_i32, |acc, (key, _value)| acc + *key)
}
