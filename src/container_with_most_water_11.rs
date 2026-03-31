/// This is very much a two pointers (left and right moving inwards) problem.
pub fn max_area(height: Vec<i32>) -> i32
{
    // Base case
    if height.len() < 1 { return 0; }
    
    let mut left = 0;
    let mut right = height.len() - 1;
    let mut current_maximum = 0;

    while right > left
    {
        let water = height[left].min(height[right]) * (right - left) as i32;

        // Check if this is a new maximum.
        if water > current_maximum
        {
            current_maximum = water;
        }

        if height[left] < height[right]
        {
            left += 1;
        }
        else
        {
            right -= 1;
        }
    }
    
    current_maximum
}
