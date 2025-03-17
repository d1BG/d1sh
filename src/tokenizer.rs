pub(crate) fn tokenize(input : &str) -> Vec<String> {
    let mut tokens = Vec::new();
    let mut curr_token = String::new();
    let mut in_quotes = false;
    let mut quoted_char = ' ';

    for c in input.chars() {
        match c {
            '"' | '\'' if !in_quotes => {
                in_quotes = true;
                quoted_char = c;
            }
            c if in_quotes && c == quoted_char => {
                in_quotes = false;
            }
            ' ' if !in_quotes => {
                if !curr_token.is_empty() {
                    tokens.push(curr_token.clone());
                    curr_token.clear();
                }
            }
            _ => { curr_token.push(c) }
        }
    }

    if !curr_token.is_empty() {
        tokens.push(curr_token);
    }

    tokens
}