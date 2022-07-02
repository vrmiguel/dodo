use std::ops::Not;

use dodo_internals::{Priority, Task};
use nom::{
    branch::alt,
    bytes::complete::{tag, take_till, take_while},
    character::complete::{char, multispace0},
    error::{ErrorKind, ParseError},
    sequence::{self, delimited, terminated},
    Err, IResult, Needed,
    Needed::Size,
};

#[cfg_attr(test, derive(Debug, PartialEq))]
struct TaskHeader<'a> {
    idx: u32,
    is_checked: bool,
    description: &'a str,
    priority: Priority,
}

pub struct Parser;

impl Parser {
    // pub fn parse(input: &str) -> IResult<&str, &str> {

    // todo!()
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

// fn rsplit_at_byte(input: &str, chr: char) -> Option<(&str, &str)> {
//     let flip = |(a, b)| (b, a);

//     input.rfind(chr).map(|idx| input.split_at(idx)).map(flip)
// }

fn parse_task_header(input: &str) -> IResult<&str, TaskHeader> {
    let (rest, idx) = parse_index(input)?;

    let (rest, is_checked) = parse_checkmark(rest)?;

    let (rest, description) = take_till(|ch| ch == '[')(rest)?;

    let (rest, priority) = parse_priority(rest)?;

    let header = TaskHeader {
        idx,
        is_checked,
        description: description.trim(),
        priority,
    };

    Ok((rest, header))
}

/// Parses number tags consisting of a number followed by a dot.
///
/// Examples: "1.", "230."
fn parse_index(input: &str) -> IResult<&str, u32> {
    let is_digit = |chr: char| {
        let chr = chr as u8;
        (0x30..=0x39).contains(&chr)
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
    let input = input.trim_start();

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

    use super::{parse_checkmark, parse_index, parse_task_header};
    use crate::parser::{parse_priority, TaskHeader};

    #[test]
    fn parses_number_tags() {
        assert_eq!(parse_index("5."), Ok(("", 5)));
        assert_eq!(parse_index("123."), Ok(("", 123)));

        assert!(parse_index("5").is_err());

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

    #[test]
    fn parses_task_header() {
        assert_eq!(
            parse_task_header("1. [ ] Fill out my tasks [HIGH]"),
            Ok((
                "",
                TaskHeader {
                    idx: 1,
                    is_checked: false,
                    description: "Fill out my tasks",
                    priority: Priority::High
                }
            ))
        );

        assert_eq!(
            parse_task_header("20.[x] Finish this test [MEDIUM]"),
            Ok((
                "",
                TaskHeader {
                    idx: 20,
                    is_checked: true,
                    description: "Finish this test",
                    priority: Priority::Medium
                }
            ))
        );
    }
}
