use crate::ast::{Expr, Statement};
use nom::{
    branch::alt,
    bytes::complete::{tag, take_until},
    character::complete::{alpha1, digit1, multispace0},
    combinator::{map, map_res},
    multi::many0,
    sequence::{delimited, preceded},
    IResult,
};

// Parse a number (e.g., `42`)
fn parse_number(input: &str) -> IResult<&str, Expr> {
    map_res(digit1, |digit_str: &str| {
        digit_str.parse::<i32>().map(Expr::Number)
    })(input)
}

// Parse a string literal (e.g., `"Hello, World!"`)
fn parse_string(input: &str) -> IResult<&str, Expr> {
    let (input, string_content) = delimited(tag("\""), take_until("\""), tag("\""))(input)?;
    Ok((input, Expr::StringLiteral(string_content.to_string())))
}

// Parse a variable (e.g., `x`)
fn parse_variable(input: &str) -> IResult<&str, Expr> {
    map(alpha1, |var: &str| Expr::Variable(var.to_string()))(input)
}

// Parse an expression
fn parse_expr(input: &str) -> IResult<&str, Expr> {
    alt((parse_number, parse_string, parse_variable))(input)
}

// Parse a var statement (`var x = <expression>;`)
fn parse_var(input: &str) -> IResult<&str, Statement> {
    let (input, _) = tag("var")(input)?;
    let (input, _) = multispace0(input)?;
    let (input, var_name) = alpha1(input)?;
    let (input, _) = multispace0(input)?;
    let (input, _) = tag("=")(input)?;
    let (input, _) = multispace0(input)?;
    let (input, expr) = parse_expr(input)?;
    let (input, _) = tag(";")(input)?;
    Ok((input, Statement::VarDecl(var_name.to_string(), expr)))
}

// Parse a val declaration (`val y = <expression>;`)
fn parse_val(input: &str) -> IResult<&str, Statement> {
    let (input, _) = tag("val")(input)?;
    let (input, _) = multispace0(input)?;
    let (input, var_name) = alpha1(input)?;
    let (input, _) = multispace0(input)?;
    let (input, _) = tag("=")(input)?;
    let (input, _) = multispace0(input)?;
    let (input, expr) = parse_expr(input)?;
    let (input, _) = tag(";")(input)?;
    Ok((input, Statement::ValDecl(var_name.to_string(), expr)))
}

// Parse an assignment statement (`x := <expression>;`)
fn parse_assign(input: &str) -> IResult<&str, Statement> {
    let (input, var_name) = alpha1(input)?;
    let (input, _) = multispace0(input)?;
    let (input, _) = tag(":=")(input)?;
    let (input, _) = multispace0(input)?;
    let (input, expr) = parse_expr(input)?;
    let (input, _) = tag(";")(input)?;
    Ok((input, Statement::Assign(var_name.to_string(), expr)))
}

// Parse a write statement (`write(<expression>);`)
fn parse_print(input: &str) -> IResult<&str, Statement> {
    let (input, _) = tag("write")(input)?;
    let (input, _) = multispace0(input)?;
    let (input, _) = tag("(")(input)?;
    let (input, expr) = parse_expr(input)?;
    let (input, _) = tag(")")(input)?;
    let (input, _) = tag(";")(input)?;
    Ok((input, Statement::Print(expr)))
}

// Parse a statement
fn parse_statement(input: &str) -> IResult<&str, Statement> {
    alt((parse_var, parse_val, parse_assign, parse_print))(input)
}

// Parse a list of statements
pub fn parse_statements(input: &str) -> IResult<&str, Vec<Statement>> {
    many0(delimited(multispace0, parse_statement, multispace0))(input)
}
