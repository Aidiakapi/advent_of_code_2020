use crate::prelude::*;

day!(18, parse => pt1, pt2);

#[derive(Clone, Eq, PartialEq)]
pub enum Expression {
    Value(i64),
    Add(Box<(Expression, Expression)>),
    Multiply(Box<(Expression, Expression)>),
}

impl Expression {
    fn evaluate(&self) -> i64 {
        match self {
            Expression::Value(v) => *v,
            Expression::Add(b) => b.0.evaluate() + b.1.evaluate(),
            Expression::Multiply(b) => b.0.evaluate() * b.1.evaluate(),
        }
    }
}

pub fn pt1(input: &str) -> Result<i64> {
    use framework::parser::*;
    fn expr(input: &str) -> IResult<Expression> {
        prim_expr(input).and_then(|(remainder, initial_expr)| {
            let op = preceded(char(' '), terminated(one_of("+*"), char(' ')));
            fold_many0(pair(op, prim_expr), initial_expr, |a, (op, b)| {
                let arg = Box::new((a, b));
                match op {
                    '+' => Expression::Add(arg),
                    '*' => Expression::Multiply(arg),
                    _ => unreachable!(),
                }
            })(remainder)
        })
    }
    fn prim_expr(input: &str) -> IResult<Expression> {
        alt((value, parenthesis))(input)
    }
    fn parenthesis(input: &str) -> IResult<Expression> {
        preceded(char('('), terminated(expr, char(')')))(input)
    }
    fn value(input: &str) -> IResult<Expression> {
        map(take_u64, |nr: u64| Expression::Value(nr as i64))(input)
    }
    let input = separated_list1(char('\n'), expr)(input).into_result()?;
    Ok(input.iter().map(Expression::evaluate).sum())
}

pub fn pt2(input: &str) -> Result<i64> {
    use framework::parser::*;
    fn expr(input: &str) -> IResult<Expression> {
        add_expr(input).and_then(|(remainder, initial_expr)| {
            fold_many0(preceded(tag(" * "), add_expr), initial_expr, |a, b| {
                Expression::Multiply(Box::new((a, b)))
            })(remainder)
        })
    }
    fn add_expr(input: &str) -> IResult<Expression> {
        prim_expr(input).and_then(|(remainder, initial_expr)| {
            fold_many0(preceded(tag(" + "), prim_expr), initial_expr, |a, b| {
                Expression::Add(Box::new((a, b)))
            })(remainder)
        })
    }
    fn prim_expr(input: &str) -> IResult<Expression> {
        alt((value, parenthesis))(input)
    }
    fn parenthesis(input: &str) -> IResult<Expression> {
        preceded(char('('), terminated(expr, char(')')))(input)
    }
    fn value(input: &str) -> IResult<Expression> {
        map(take_u64, |nr: u64| Expression::Value(nr as i64))(input)
    }
    let input = separated_list1(char('\n'), expr)(input).into_result()?;
    Ok(input.iter().map(Expression::evaluate).sum())
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
