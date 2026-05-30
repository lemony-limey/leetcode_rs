/// The matrices are COLUMN-MAJOR.
///
/// The string "PAYPALISHIRING" with num_rows = 3 would become:
///
/// P   A   H   N
/// A P L S I I G
/// Y   I   R
///
/// so the result line-by-line is "PAHNAPLSIIGYIR".
///
/// -----
///
/// Consider the same input, but with num_rows = 4:
///
/// P     I     N
/// A   L S   I G
/// Y A   H R
/// P     I
///
/// -----
///
/// Consider the same input, but with num_rows = 5:
///
/// P       H
/// A     S I
/// Y   I   R
/// P L     I G
/// A       N
///
///
/// The first and last rows start from characters at indices
/// 0 and (num_rows - 1) respectively, and have a stride of
/// num_rows.
///
/// TODO: Bugged, something about the middle indices.
pub fn convert(s: String, num_rows: i32) -> String
{
    // Base case.
    if num_rows == 1
    {
        return s;
    }

    let chars: Vec<char> = s.chars().collect();

    // Number to add to rows in between
    let offset = (num_rows * 2) - 2;
    let mut result = String::new();

    // if 4 rows, to get thing in row at index i = 2, also get thing at current + offset - i
    for i in 0..num_rows
    {
        let mut current_index = i as usize;
        while current_index < s.len()
        {
            result.push(chars[current_index]);

            if i > 0 && i < num_rows - 1
            {
                let second = current_index + (offset - i - 1) as usize;

                // Handle case where current_index and current_index + offset - i
                //   are equal.
                if second < chars.len()  // is within bounds
                    && second != current_index
                // has not already been added this iteration
                {
                    result.push(chars[second]);
                }
            }

            current_index += offset as usize;
        }
    }

    result
}
