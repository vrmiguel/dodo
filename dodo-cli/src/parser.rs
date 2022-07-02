use std::ops::Not;

use dodo_internals::Priority;
use nom::{
    branch::alt,
    bytes::complete::{tag, take_while},
    character::complete::{char, multispace0},
    error::{ErrorKind, ParseError},
    sequence::{self, delimited},
    Err, IResult, Needed,
    Needed::Size,
};

pub struct Parser;

impl Parser {
    // pub fn parse(input: &str) -> ParseResult<Task> {
    //     todo!()
    // }
}

fn ws<'a, F: 'a, O, E: ParseError<&'a str>>(
    inner: F,
) -> impl FnMut(&'a str) -> IResult<&'a str, O, E>
where
    F: Fn(&'a str) -> IResult<&'a str, O, E>,
{
    sequence::delimited(multispace0, inner, multispace0)
}

/// Parses number tags consisting of a number followed by a dot.
///
/// Examples: "1.", "230."
fn parse_number_tag(input: &str) -> IResult<&str, u32> {
    let is_digit = |chr: char| {
        let chr = chr as u8;
        chr >= 0x30 && chr <= 0x39
    };

    let (rest, digits) = take_while(is_digit)(input)?;

    // TODO: turn this into an actual error
    assert!(digits.is_empty().not());

    let (rest, _) = char('.')(rest)?;

    let number = digits
        .parse()
        .expect("Digits somehow are an invalid number");

    Ok((rest, number))
}

/// Parses checkmarks consisting of a "x", "X" or " " within brackets.
///
/// Examples: "[x]", "[X]", "[ ]"
fn parse_checkmark(input: &str) -> IResult<&str, bool> {
    let (rest, chr) = delimited(
        char('['),
        alt((char('X'), char('x'), char(' '))),
        char(']'),
    )(input)?;

    let is_checked = matches!(chr, 'x' | 'X');

    Ok((rest, is_checked))
}

/// Parses priority declarations.
///
/// Examples: "[HIGH]", "[MEDIUM]", "[LOW]"
fn parse_priority(input: &str) -> IResult<&str, Priority> {
    let (rest, priority): (&str, &str) = delimited(
        tag("["),
        alt((
            tag("HIGH"),
            tag("MEDIUM"),
            tag("LOW"),
            tag("high"),
            tag("medium"),
            tag("low"),
        )),
        tag("]"),
    )(input)?;

    // TODO: error mgmt
    let priority = Priority::from_str(priority).expect("invalid priority text");

    Ok((rest, priority))
}

#[cfg(test)]
mod tests {
    use dodo_internals::Priority;

    use super::{parse_checkmark, parse_number_tag};
    use crate::parser::parse_priority;

    #[test]
    fn parses_number_tags() {
        assert_eq!(parse_number_tag("5."), Ok(("", 5)));
        assert_eq!(parse_number_tag("123."), Ok(("", 123)));

        assert!(parse_number_tag("5").is_err());

        // assert!(parse_number_tag(".").is_err());
    }

    #[test]
    fn parses_checkbox() {
        assert_eq!(parse_checkmark("[x]"), Ok(("", true)));
        assert_eq!(parse_checkmark("[X]"), Ok(("", true)));
        assert_eq!(parse_checkmark("[ ]"), Ok(("", false)));

        assert!(parse_checkmark("[]").is_err());
        assert!(parse_checkmark("{x}").is_err());
    }

    #[test]
    fn parses_priority() {
        assert_eq!(parse_priority("[HIGH]"), Ok(("", Priority::High)));

        assert_eq!(
            parse_priority("[MEDIUM]"),
            Ok(("", Priority::Medium))
        );

        assert_eq!(parse_priority("[LOW]"), Ok(("", Priority::Low)));

        // TODO: test error cases
    }
}
