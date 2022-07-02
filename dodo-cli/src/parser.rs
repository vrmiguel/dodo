use std::ops::Not;

use dodo_internals::{
    utils::today, Checkbox, Checklist, Priority, Task,
};
use nom::{
    branch::alt,
    bytes::complete::{tag, take_till, take_while},
    character::{
        complete::{char, multispace0},
        is_newline,
    },
    error::{ErrorKind, ParseError},
    multi::many0,
    sequence::{self, delimited, terminated},
    Err, IResult, Needed,
    Needed::Size,
};

#[derive(Debug, PartialEq)]
// #[cfg_attr(test, derive(Debug, PartialEq))]
struct TaskHeader<'a> {
    idx: u32,
    is_checked: bool,
    name: &'a str,
    priority: Priority,
}

impl TaskHeader<'_> {
    pub fn with_checkboxes(
        self,
        checkboxes: Vec<Checkbox>,
    ) -> Task {
        Task {
            name: self.name.to_owned(),
            is_done: self.is_checked,
            // TODO: figure this out
            creation_date: today(),
            // TODO: figure this out
            due_date: None,
            priority: self.priority,
            checklist: Checklist::with_checkboxes(checkboxes),
        }
    }
}

pub struct Parser;

impl Parser {
    pub fn parse(input: &str) -> IResult<&str, Task> {
        let (rest, header) = parse_task_header(input)?;

        let (rest, checkboxes) = many0(parse_checkbox)(rest)?;

        Ok((rest, header.with_checkboxes(checkboxes)))
    }
}

fn ws<'a, F: 'a, O, E: ParseError<&'a str>>(
    inner: F,
) -> impl FnMut(&'a str) -> IResult<&'a str, O, E>
where
    F: Fn(&'a str) -> IResult<&'a str, O, E>,
{
    sequence::delimited(multispace0, inner, multispace0)
}

// fn rsplit_at_byte(input: &str, chr: char) -> Option<(&str,
// &str)> {     let flip = |(a, b)| (b, a);

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
        name: description.trim(),
        priority,
    };

    Ok((rest, header))
}

/// Parses a [`Checkbox`], which consists of an asterisk, a
/// checkmark and a description.
///
/// Examples:
/// '* [x] Finish this doctest'
fn parse_checkbox(input: &str) -> IResult<&str, Checkbox> {
    let input = input.trim_start();

    let (rest, _asterisk) = ws(char('*'))(input)?;

    let (rest, is_checked) = parse_checkmark(rest)?;

    let (rest, description) = take_till(|ch| ch == '\n')(rest)?;

    let checkbox =
        Checkbox::with_description(description.trim().into())
            .with_status(is_checked);

    Ok((rest, checkbox))
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

/// Parses checkmarks consisting of a "x", "X" or " " within
/// brackets.
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
    let priority = Priority::from_str(priority)
        .expect("invalid priority text");

    Ok((rest, priority))
}

#[cfg(test)]
mod tests {
    use dodo_internals::{
        utils::today, Checkbox, Priority, Task,
    };

    use super::{
        parse_checkmark, parse_index, parse_task_header, Parser,
    };
    use crate::parser::{
        parse_checkbox, parse_priority, TaskHeader,
    };

    #[test]
    fn parses_number_tags() {
        assert_eq!(parse_index("5."), Ok(("", 5)));
        assert_eq!(parse_index("123."), Ok(("", 123)));

        assert!(parse_index("5").is_err());

        // assert!(parse_number_tag(".").is_err());
    }

    #[test]
    fn parses_checkmark() {
        assert_eq!(parse_checkmark("[x]"), Ok(("", true)));
        assert_eq!(parse_checkmark("[X]"), Ok(("", true)));
        assert_eq!(parse_checkmark("[ ]"), Ok(("", false)));

        assert!(parse_checkmark("[]").is_err());
        assert!(parse_checkmark("{x}").is_err());
    }

    #[test]
    fn parses_checkboxes() {
        assert_eq!(
            parse_checkbox("  * [x] Finish this test"),
            Ok((
                "",
                Checkbox::with_description(
                    "Finish this test".into()
                )
                .with_status(true)
            ))
        );

        assert_eq!(
            parse_checkbox("*[ ] Finish this test\n"),
            Ok((
                "\n",
                Checkbox::with_description(
                    "Finish this test".into()
                )
                .with_status(false)
            ))
        );
    }

    #[test]
    fn parses_priority() {
        assert_eq!(
            parse_priority("[HIGH]"),
            Ok(("", Priority::High))
        );

        assert_eq!(
            parse_priority("[MEDIUM]"),
            Ok(("", Priority::Medium))
        );

        assert_eq!(
            parse_priority("[LOW]"),
            Ok(("", Priority::Low))
        );

        // TODO: test error cases
    }

    #[test]
    fn parse_task_1() {
        let task = "1. [ ] Fill out my tasks [HIGH]\n  * [ ] Figure out how to use dodo\n";

        assert_eq!(
            Parser::parse(task),
            Ok((
                "\n",
                Task {
                    name: "Fill out my tasks".into(),
                    is_done: false,
                    creation_date: today(),
                    due_date: None,
                    priority: Priority::High,
                    checklist: [Checkbox::with_description(
                        "Figure out how to use dodo".into()
                    )]
                    .into_iter()
                    .collect()
                }
            ))
        );
    }

    #[test]
    fn parse_task_2() {
        let task = "1. [ ] Fill out my tasks [HIGH]\n  * [ ] Figure out how to use dodo\n* [x] Make this test pass\n";

        assert_eq!(
            Parser::parse(task),
            Ok((
                "\n",
                Task {
                    name: "Fill out my tasks".into(),
                    is_done: false,
                    creation_date: today(),
                    due_date: None,
                    priority: Priority::High,
                    checklist: [
                        Checkbox::with_description(
                            "Figure out how to use dodo".into()
                        ),
                        Checkbox::with_description(
                            "Make this test pass".into()
                        )
                        .with_status(true)
                    ]
                    .into_iter()
                    .collect()
                }
            ))
        );
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
                    name: "Fill out my tasks",
                    priority: Priority::High
                }
            ))
        );

        assert_eq!(
            parse_task_header(
                "20.[x] Finish this test [MEDIUM]"
            ),
            Ok((
                "",
                TaskHeader {
                    idx: 20,
                    is_checked: true,
                    name: "Finish this test",
                    priority: Priority::Medium
                }
            ))
        );
    }
}
