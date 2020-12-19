use crate::prelude::*;

day!(18, parse => pt1, pt2);

fn value(input: &str) -> framework::parser::IResult<u64> {
    use framework::parser::*;
    if input.is_empty() {
        return Err(nom::Err::Error(AocParseError(
            input,
            AocErrorKind::TakeUnsigned(TakeIntErrorKind::Empty),
        )));
    }
    let b = input.as_bytes()[0];
    if b < b'1' || b > b'9' {
        return Err(nom::Err::Error(AocParseError(
            input,
            AocErrorKind::TakeUnsigned(TakeIntErrorKind::InvalidCharacter),
        )));
    }
    Ok((&input[1..], (b - b'0') as u64))
}

pub fn pt1(input: &str) -> Result<u64> {
    use framework::parser::*;
    fn expr(input: &str) -> IResult<u64> {
        prim_expr(input).and_then(|(remainder, initial_value)| {
            let op = preceded(char(' '), terminated(one_of("+*"), char(' ')));
            fold_many0(pair(op, prim_expr), initial_value, |a, (op, b)| match op {
                '+' => a + b,
                '*' => a * b,
                _ => unreachable!(),
            })(remainder)
        })
    }
    fn prim_expr(input: &str) -> IResult<u64> {
        alt((value, parenthesis))(input)
    }
    fn parenthesis(input: &str) -> IResult<u64> {
        preceded(char('('), terminated(expr, char(')')))(input)
    }
    let input = separated_list1(char('\n'), expr)(input).into_result()?;
    Ok(input.into_iter().sum())
}

pub fn pt2(input: &str) -> Result<u64> {
    use framework::parser::*;
    fn expr(input: &str) -> IResult<u64> {
        add_expr(input).and_then(|(remainder, initial_expr)| {
            fold_many0(preceded(tag(" * "), add_expr), initial_expr, |a, b| a * b)(remainder)
        })
    }
    fn add_expr(input: &str) -> IResult<u64> {
        prim_expr(input).and_then(|(remainder, initial_expr)| {
            fold_many0(preceded(tag(" + "), prim_expr), initial_expr, |a, b| a + b)(remainder)
        })
    }
    fn prim_expr(input: &str) -> IResult<u64> {
        alt((value, parenthesis))(input)
    }
    fn parenthesis(input: &str) -> IResult<u64> {
        preceded(char('('), terminated(expr, char(')')))(input)
    }
    let input = separated_list1(char('\n'), expr)(input).into_result()?;
    Ok(input.into_iter().sum())
}

pub fn parse(input: &str) -> Result<&str> {
    Ok(input)
}

standard_tests!(
    parse []
    pt1 [
        "1 + 2 * 3 + 4 * 5 + 6" => 71
        "2 * 3 + (4 * 5)" => 26
        "5 + (8 * 3 + 9 + 3 * 4 * 3)" => 437
        "5 * 9 * (7 * 3 * 3 + 9 * 3 + (8 + 6 * 4))" => 12240
        "((2 + 4 * 9) * (6 + 9 * 8 + 6) + 6) + 2 + 4 * 2" => 13632
    ]
    pt2 [
        "1 + 2 * 3 + 4 * 5 + 6" => 231
        "1 + (2 * 3) + (4 * (5 + 6))" => 51
        "2 * 3 + (4 * 5)" => 46
        "5 + (8 * 3 + 9 + 3 * 4 * 3)" => 1445
        "5 * 9 * (7 * 3 * 3 + 9 * 3 + (8 + 6 * 4))" => 669060
        "((2 + 4 * 9) * (6 + 9 * 8 + 6) + 6) + 2 + 4 * 2" => 23340
    ]
);
