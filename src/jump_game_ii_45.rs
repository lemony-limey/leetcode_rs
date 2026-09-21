pub fn jump(nums: Vec<i32>) -> i32
{
    let n = nums.len();

    // Base cases: if 0 or 1 then no moves are required.
    if n <= 1 { return 0; }
    // Base case: if length is 2, then only 1 jump is necessary as it is guaranteed to be
    // possible to reach the end of the list.
    else if n == 2 { return 1; }

    let mut i = 0;
    let mut jump_counter = 1;

    // Two pointers over the array. (O(n) time, because each element only needs to be traversed once?).
    while (i + nums[i] as usize) < n - 1
    {
        // max_index is where the left pointer i will move to after searching.
        let mut max_index = i + 1;
        let mut max_individual_value: usize = nums[max_index] as usize;
        let mut max_movement_potential: usize = nums[max_index] as usize + 1;

        // Do not ever land on a 0.
        // Starting from i + 2 because no need to search i, and assuming i + 1 is current max.
        for j in 2..=(nums[i] as usize)
        {
            let x = i + j;

            // For each index x = (i + j), calculate j + nums[x] and store
            // the x where this is largest.
            // If equal to the previous movement maximum, choose the larger index
            // to make more forward progress.

            // current_index = x;
            let current_value = nums[x] as usize;
            let current_movement_potential = j + current_value;

            // Total potential is what we care about most first.
            // If multiple options have the same total movement potential, choose the
            // cell with the highest index so we can skip over pointless moves.
            if current_movement_potential >= max_movement_potential
            {
                max_index = x;
                max_movement_potential = current_movement_potential;
            }
        }

        // Move left pointer to new position.
        i = max_index;

        // Count the fact we will make another jump, then decide where to jump.
        jump_counter += 1;
    }

    jump_counter
}

#[cfg(test)]
mod tests
{
    use crate::jump_game_45::jump;

    #[test]
    fn first()
    {
        let nums: Vec<i32> = vec![1, 2, 1, 1, 1];
        assert_eq!(jump(nums), 3);
    }

    #[test]
    fn second()
    {
        let nums: Vec<i32> = vec![2, 3, 1, 1, 4];
        assert_eq!(jump(nums), 2);
    }

    #[test]
    fn third()
    {
        let nums: Vec<i32> = vec![4, 1, 1, 3, 1, 1, 1];
        assert_eq!(jump(nums), 2);
    }

    #[test]
    fn fourth()
    {
        let nums: Vec<i32> = vec![10,9,8,7,6,5,4,3,2,1,1,0];
        assert_eq!(jump(nums), 2);
    }
}
