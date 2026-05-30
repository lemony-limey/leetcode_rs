pub fn max_profit(prices: Vec<i32>) -> i32
{
    if prices.len() <= 1
    {
        return 0;
    }

    // Profit will remain 0 if the value at slow is always greater than the
    // value at fast.
    let mut left = 0;
    let mut right = 0;
    let mut profit = 0;

    while right < prices.len()
    {
        // If makes a profit
        if prices[right] > prices[left]
        {
            // Update max profit if necessary
            if prices[right] - prices[left] > profit
            {
                profit = prices[right] - prices[left];
            }
        }
        else
        {
            left = right;
        }

        right += 1;
    }

    profit
}
