pub fn is_valid(s: String) -> bool
{
    // Parenthesis can only be matched if there are an even number of characters.
    if s.len() % 2 == 1 { return false; }
    
    let mut s = s;
    let mut stack: Vec<char> = Vec::new();

    while let Some(c) = s.pop()
    {
        if c == '}' || c == ']' || c == ')' 
        {
            stack.push(c);
        }
        else if c == '{' || c == '[' || c == '('
        {
            let Some(matching) = stack.pop() else {
                return false;
            };
            match (c, matching)
            {
                ('{', '}') | ('(', ')') | ('[', ']') => {}
                _ => return false
            }
        }
    }

    stack.is_empty()
}
