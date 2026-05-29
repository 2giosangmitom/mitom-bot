use thiserror::Error;

#[derive(Error, Debug, PartialEq)]
pub enum CalculateError {
    #[error("unexpected {character} at {index}")]
    InvalidExpression { character: char, index: usize },

    #[error("unmatched {paren_type} at {index}")]
    UnmatchedParentheses { paren_type: String, index: usize },

    #[error("division by zero at {index}")]
    DivisionByZero { index: usize },
}

pub(crate) struct Parser {
    chars: Vec<char>,
    pos: usize,
}

impl Parser {
    pub fn new(input: &str) -> Self {
        Self {
            chars: input.chars().collect(),
            pos: 0,
        }
    }

    pub fn parse(&mut self) -> Result<i32, CalculateError> {
        let result = self.parse_expr()?;

        self.skip_whitespace();

        match self.peek() {
            Some(')') => Err(CalculateError::UnmatchedParentheses {
                paren_type: "closing".to_string(),
                index: self.pos,
            }),
            Some(c) => Err(CalculateError::InvalidExpression {
                character: c,
                index: self.pos,
            }),
            None => Ok(result),
        }
    }

    /// expr := term (('+' | '-') term)*
    fn parse_expr(&mut self) -> Result<i32, CalculateError> {
        let mut value = self.parse_term()?;

        loop {
            self.skip_whitespace();

            match self.peek() {
                Some('+') => {
                    self.consume();
                    value += self.parse_term()?;
                }
                Some('-') => {
                    self.consume();
                    value -= self.parse_term()?;
                }
                _ => break,
            }
        }

        Ok(value)
    }

    /// term := factor (('*' | '/') factor)*
    fn parse_term(&mut self) -> Result<i32, CalculateError> {
        let mut value = self.parse_factor()?;

        loop {
            self.skip_whitespace();

            match self.peek() {
                Some('*') => {
                    self.consume();
                    value *= self.parse_factor()?;
                }
                Some('/') => {
                    let op_index = self.pos;

                    self.consume();

                    let rhs = self.parse_factor()?;

                    if rhs == 0 {
                        return Err(CalculateError::DivisionByZero { index: op_index });
                    }

                    value /= rhs;
                }
                _ => break,
            }
        }

        Ok(value)
    }

    /// factor := ('+' | '-') factor
    ///         | number
    ///         | '(' expr ')'
    fn parse_factor(&mut self) -> Result<i32, CalculateError> {
        self.skip_whitespace();

        match self.peek() {
            Some('+') => {
                self.consume();
                self.parse_factor()
            }

            Some('-') => {
                self.consume();
                Ok(-self.parse_factor()?)
            }

            Some('(') => {
                let open_index = self.pos;

                self.consume();

                let value = self.parse_expr()?;

                self.skip_whitespace();

                match self.peek() {
                    Some(')') => {
                        self.consume();
                        Ok(value)
                    }
                    _ => Err(CalculateError::UnmatchedParentheses {
                        paren_type: "closing".to_string(),
                        index: open_index + 1,
                    }),
                }
            }

            Some(c) if c.is_ascii_digit() => Ok(self.parse_number()),

            Some(c) => Err(CalculateError::InvalidExpression {
                character: c,
                index: self.pos,
            }),

            None => Err(CalculateError::InvalidExpression {
                character: '\0',
                index: self.pos,
            }),
        }
    }

    fn parse_number(&mut self) -> i32 {
        let mut value = 0;

        while let Some(c) = self.peek() {
            if !c.is_ascii_digit() {
                break;
            }

            value = value * 10 + (c as i32 - '0' as i32);
            self.pos += 1;
        }

        value
    }

    fn skip_whitespace(&mut self) {
        while matches!(self.peek(), Some(c) if c.is_whitespace()) {
            self.pos += 1;
        }
    }

    fn peek(&self) -> Option<char> {
        self.chars.get(self.pos).copied()
    }

    fn consume(&mut self) -> Option<char> {
        let c = self.peek()?;
        self.pos += 1;
        Some(c)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    #[rstest]
    #[case("1+1", 2)]
    #[case("1-1", 0)]
    #[case("1-1+2", 2)]
    #[case("+4-2+10", 12)]
    #[case("-15+10", -5)]
    #[case("12+8-(10+2)", 8)]
    #[case("2-(5-6)", 3)]
    #[case("2-(5-6)+3", 6)]
    #[case("2-(5-6)+3-(4+2)", 0)]
    #[case("2+3*4", 14)]
    #[case("(2+3)*4", 20)]
    #[case("10/2+5", 10)]
    #[case("2*(3+4)", 14)]
    #[case("2*(3-(4+1))", -4)]
    #[case("-1+-3", -4)]
    #[case("--5", 5)]
    #[case("-(2+3)", -5)]
    #[case("3*-2", -6)]
    fn test_calculate(#[case] expression: &str, #[case] expected: i32) {
        let actual = Parser::new(expression).parse().unwrap();
        assert_eq!(actual, expected);
    }

    #[rstest]
    #[case(
        "1+n2",
        CalculateError::InvalidExpression {
            character: 'n',
            index: 2
        }
    )]
    #[case(
        "1-2)",
        CalculateError::UnmatchedParentheses {
            paren_type: "closing".to_string(),
            index: 3
        }
    )]
    #[case(
        "(1-2",
        CalculateError::UnmatchedParentheses {
            paren_type: "closing".to_string(),
            index: 1
        }
    )]
    #[case(
        "1/0",
        CalculateError::DivisionByZero {
            index: 1
        }
    )]
    #[case(
        "10/(5-5)",
        CalculateError::DivisionByZero {
            index: 2
        }
    )]
    fn test_calculate_error(#[case] expression: &str, #[case] expected: CalculateError) {
        let actual = Parser::new(expression).parse().unwrap_err();
        assert_eq!(actual, expected);
    }
}
