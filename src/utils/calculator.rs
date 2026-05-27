use std::collections::VecDeque;

pub(crate) fn calculate(expression: &str) -> anyhow::Result<i32> {
    let mut stack: VecDeque<(i32, i32)> = VecDeque::new();
    let n = expression.len();
    let expr_bytes = expression.as_bytes();
    let mut result: i32 = 0;
    let mut curr_num: i32 = 0;
    let mut curr_sign: i32 = 1;
    let mut i = 0;

    while i < n {
        if expr_bytes[i] == b'-' {
            curr_sign = -1;
            i += 1;
        } else if expr_bytes[i] == b'+' {
            curr_sign = 1;
            i += 1;
        } else if expr_bytes[i] == b'(' {
            result += curr_num;
            stack.push_back((result, curr_sign));
            result = 0;
            curr_num = 0;
            curr_sign = 1;
            i += 1;
        } else if expr_bytes[i] == b')' {
            let (last_result, last_sign) = stack.pop_back().unwrap();
            result = last_result + last_sign * result;
            i += 1;
        } else if expr_bytes[i].is_ascii_digit() {
            while i < n
                && let Some(d) = char::from(expr_bytes[i]).to_digit(10)
            {
                curr_num = curr_num * 10 + d as i32;
                i += 1;
            }
            result += curr_sign * curr_num;
            curr_num = 0;
            curr_sign = 1;
        } else if expr_bytes[i].is_ascii_whitespace() {
            i += 1;
        } else {
            return Err(anyhow::anyhow!(
                "Invalid character: {}",
                expr_bytes[i] as char
            ));
        }
    }

    Ok(result)
}

#[cfg(test)]
mod tests {
    use super::calculate;
    use rstest::rstest;

    #[rstest]
    #[case("1+1", anyhow::Ok(2))]
    #[case("1-1", anyhow::Ok(0))]
    #[case("1-1+2", anyhow::Ok(2))]
    #[case("+4-2+10", anyhow::Ok(12))]
    #[case("-15+10", anyhow::Ok(-5))]
    #[case("12+8-(10+2)", anyhow::Ok(8))]
    #[case("2-(5-6)", anyhow::Ok(3))]
    #[case("2-(5-6)+3", anyhow::Ok(6))]
    #[case("2-(5-6)+3-(4+2)", anyhow::Ok(0))]
    fn test_calculate(#[case] expression: &str, #[case] expected: anyhow::Result<i32>) {
        let actual = calculate(expression);
        assert_eq!(actual.unwrap(), expected.unwrap());
    }
}
