//! File that parses user input and breaks it down into tokens

#[derive(Debug, Clone, Copy)]
pub enum ParameterType
{
    Generic,
    String,
    Number,
}

#[derive(Debug, Clone, Copy)]
pub enum TokenType
{
    Error,
    Command,
    Parameter(ParameterType),
}

#[derive(Debug, Clone, Copy)]
pub struct Token {
    range: (usize, usize),
    token: TokenType,
}

pub fn parse(line: &String) -> Vec::<Token> {
    let mut result = Vec::new();

    let mut start = 0;
    let mut end = 0;
    let mut current_token = TokenType::Error;

    let str_as_bytes = line.as_bytes();

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

    start = end;

    let mut iter = str_as_bytes[start ..].iter();
    let mut parsing = false;

    loop {
        end += 1;

        match iter.next() {
            Some(current) => match current {
                b'"' => println!("Found a thingy"),
                b'0' ..= b'9' => {
                    current_token = TokenType::Parameter(ParameterType::Number);
                    parsing = true;
                    continue;
                },
                _ => {
                    if current.is_ascii_whitespace() && parsing {
                        parsing = false;
                        result.push(Token { range: (start, end), token: current_token });
                    }
                },
            },
            None => {
                if parsing {
                    result.push(Token { range: (start, end - 1), token: current_token });
                }
                break;
            },
        }

        start = end;
    }

        /* while current.is_ascii_whitespace() {
            i += 1;

            current = match str_as_bytes.get(i) {
                Some(c) => {
                    end += 1;
                    *c
                },
                None => {
                    i += 1;
                    result.push(Token { range: (iter, end), token: TokenType::Whitespace });
                    break;
                },
            };
        }

        while current.is_ascii_digit() {
            i += 1;

            current = match str_as_bytes.get(i) {
                Some(c) => {
                    end += 1;
                    *c
                },
                None => {
                    i += 1;
                    result.push(Token { range: (iter, end), token: TokenType::Parameter(ParameterType::Number) });
                    break;
                },
            };
        }

        iter = end; */

    result
}
