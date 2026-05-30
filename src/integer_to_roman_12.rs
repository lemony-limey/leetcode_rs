/// M = 1000
/// D = 500
/// C = 100
/// L = 50
/// X = 10
/// V = 5
/// I = 1
pub fn int_to_roman(num: i32) -> String
{
    let mut result = String::new();
    let mut num_remaining = num;
    let mappings = [
        (1000, "M"),
        (900, "CM"),
        (500, "D"),
        (400, "CD"),
        (100, "C"),
        (90, "XC"),
        (50, "L"),
        (40, "XL"),
        (10, "X"),
        (9, "IX"),
        (5, "V"),
        (4, "IV"),
        (1, "I"),
    ];

    for (val, symbols) in mappings
    {
        let number_of_symbols = num_remaining / val;
        for _ in 0..number_of_symbols
        {
            result.push_str(symbols);
            num_remaining -= val;
        }
    }

    result
}
