use thiserror::Error;

#[derive(Error, Debug, PartialEq)]
pub(crate) enum CalculateError {
    #[error("unexpected {character} at {index}")]
    InvalidExpression { character: char, index: usize },

    #[error("unmatched {paren_type} at {index}")]
    UnmatchedParentheses { paren_type: String, index: usize },
}

pub(crate) fn calculate(expression: &str) -> Result<i32, CalculateError> {
    let mut stack: Vec<(i32, i32)> = Vec::new();
    let n = expression.len();
    let expr_bytes = expression.as_bytes();
    let mut result: i32 = 0;
    let mut sign: i32 = 1;
    let mut i = 0;
    let mut require_digit = false;

    while i < n {
        match expr_bytes[i] {
            b'-' | b'+' => {
                if require_digit {
                    return Err(CalculateError::InvalidExpression {
                        character: expr_bytes[i] as char,
                        index: i,
                    });
                }
                sign = if expr_bytes[i] == b'-' { -1 } else { 1 };
                require_digit = true;
            }
            b'(' => {
                stack.push((result, sign));
                result = 0;
                sign = 1;
                require_digit = false;
            }
            b')' => {
                if let Some((last_result, last_sign)) = stack.pop() {
                    result = last_result + last_sign * result;
                    require_digit = false;
                } else {
                    return Err(CalculateError::UnmatchedParentheses {
                        paren_type: "closing".to_string(),
                        index: i,
                    });
                }
            }
            b'0'..=b'9' => {
                let mut curr_num: i32 = 0;
                while i < n
                    && let Some(d) = char::from(expr_bytes[i]).to_digit(10)
                {
                    curr_num = curr_num * 10 + d as i32;
                    i += 1;
                }
                result += sign * curr_num;
                sign = 1;
                require_digit = false;
                continue;
            }
            _ => {
                if expr_bytes[i].is_ascii_whitespace() {
                    i += 1;
                } else {
                    return Err(CalculateError::InvalidExpression {
                        character: expr_bytes[i] as char,
                        index: i,
                    });
                }
            }
        }
        i += 1;
    }

    if !stack.is_empty() {
        return Err(CalculateError::UnmatchedParentheses {
            paren_type: "closing".to_string(),
            index: n,
        });
    }

    Ok(result)
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    #[rstest]
    #[case("1+1", Ok(2))]
    #[case("1-1", Ok(0))]
    #[case("1-1+2", Ok(2))]
    #[case("+4-2+10", Ok(12))]
    #[case("-15+10", Ok(-5))]
    #[case("12+8-(10+2)", Ok(8))]
    #[case("2-(5-6)", Ok(3))]
    #[case("2-(5-6)+3", Ok(6))]
    #[case("2-(5-6)+3-(4+2)", Ok(0))]
    fn test_calculate(#[case] expression: &str, #[case] expected: Result<i32, CalculateError>) {
        let actual = calculate(expression);
        assert_eq!(actual.unwrap(), expected.unwrap());
    }

    #[rstest]
    #[case("-1+-3", Err(CalculateError::InvalidExpression { character: '-', index: 3 }))]
    #[case("1+n2", Err(CalculateError::InvalidExpression { character: 'n', index: 2 }))]
    #[case("1-2)", Err(CalculateError::UnmatchedParentheses { paren_type: "closing".to_string(), index: 3 }))]
    #[case("(1-2", Err(CalculateError::UnmatchedParentheses { paren_type: "closing".to_string(), index: 4 }))]
    #[case("1-2)+3", Err(CalculateError::UnmatchedParentheses { paren_type: "closing".to_string(), index: 3 }))]
    fn test_calculate_error(
        #[case] expression: &str,
        #[case] expected: Result<i32, CalculateError>,
    ) {
        let actual = calculate(expression);
        assert_eq!(actual.unwrap_err(), expected.unwrap_err());
    }
}
