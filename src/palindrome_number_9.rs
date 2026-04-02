pub fn is_palindrome(x: i32) -> bool
{
    // x cannot be a palindrome if negative or if it is a number greater than
    // 0 that happens to end with a 0.
    if x < 0 || (x % 10 == 0 && x != 0) { return false; }

    let mut num = x;
    let mut reversed = 0;

    // Reverse the order of the digits.
    while num != 0
    {
        reversed = (reversed * 10) + (num % 10);
        num /= 10;
    }

    // If the forward and reverse numbers are equal, we have a match.
    x == reversed
}