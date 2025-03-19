use std::env;

pub(crate) fn tokenize(input : &str) -> Vec<String> {
    let mut tokens = Vec::new();
    let mut curr_token = String::new();
    let mut in_quotes = false;
    let mut quoted_char = ' ';
    let mut escape = false;

    for c in input.chars() {
        if escape {
            curr_token.push(c);
            escape = false;
        } else {
            match c {
                '\\' => escape = true,
                '"' | '\'' if !in_quotes => {
                    in_quotes = true;
                    quoted_char = c;
                }
                c if in_quotes && c == quoted_char => {
                    in_quotes = false;
                }
                ' ' if !in_quotes && !curr_token.is_empty() => {
                    if curr_token.to_string().starts_with('$') {
                        let envvar = env::var(curr_token.clone().strip_prefix('$').unwrap().to_string()).unwrap();
                        tokens.push(envvar);
                        curr_token.clear();
                    } else {
                        tokens.push(curr_token.clone());
                        curr_token.clear();
                    }
                }
                _ => { curr_token.push(c) }
            }
        }
    }
    if !curr_token.is_empty() && !in_quotes && curr_token.to_string().starts_with('$') {
        let envvar = env::var(curr_token.clone().strip_prefix('$').unwrap().to_string()).unwrap();
        tokens.push(envvar);
    } else if !curr_token.is_empty() {
        tokens.push(curr_token);
    }

    tokens
}