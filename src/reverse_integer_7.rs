use std::str::FromStr;

pub fn reverse(x: i32) -> i32
{
    let mut x = x.to_string();

    let mut is_negative = false;
    if &x[0..1] == "-"
    {
        is_negative = true;
        x.remove(0);
    }

    let mut result = String::new();

    // For each byte in the string, swap places
    for digit in x.as_bytes().iter().rev()
    {
        result.push(*digit as char);
    }

    if let Ok(result) = i32::from_str(&*result)
    {
        if is_negative { -result } else { result }
    }
    else
    {
        0
    }
}
