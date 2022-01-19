//! File that parses user input and breaks it down into tokens

#[derive(Debug)]
pub enum ParameterType
{
    String,
    Number,
}

#[derive(Debug)]
pub enum TokenType
{
    Whitespace,
    Command,
    Parameter(ParameterType),
}

#[derive(Debug)]
pub struct Token {
    range: (usize, usize),
    token: TokenType,
}

pub fn parse(line: &String) -> Vec::<Token> {
    let mut result = Vec::new();

    let mut start = 0;
    let mut end;
    // let mut current_token = TokenType::Whitespace;

    let str_as_bytes = line.as_bytes();

    let mut skip_whitespace = || {
        while str_as_bytes[start].is_ascii_whitespace() {
            start += 1;
        }
    };

    // First, try to skip all whitespace
    skip_whitespace();

    // Assign the end of our range to the start of the first non-whitespace
    // char
    end = start;

    // After we've skipped all potential whitespace it's time to parse
    // the main command
    while !str_as_bytes[end].is_ascii_whitespace() {
        end += 1;
        if str_as_bytes.get(end).is_none() {
            break;
        }
    }

    // Now that we've read our command we can place it in our result
    result.push(Token { range: (start, end), token: TokenType::Command });

    for i in 0 .. line.len() {
        let current = str_as_bytes[i];
        let lookahead = str_as_bytes.get(i + 1);
    }

    result
}
