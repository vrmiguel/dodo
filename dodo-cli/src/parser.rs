use dodo_internals::{
    utils::today, Checkbox, Checklist, Priority, Task, TaskSet,
};
use nom::{
    branch::alt,
    bytes::{
        complete::{tag, take_till, take_while},
        streaming::take,
    },
    character::complete::{char, multispace0},
    error::ParseError,
    multi::many0,
    sequence::{self, delimited},
    IResult,
};

use crate::Result;

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

fn parse_task(input: &str) -> IResult<&str, Task> {
    let input = input.trim_start();

    let (rest, header) = parse_task_header(input)?;

    let (rest, checkboxes) = many0(parse_checkbox)(rest)?;

    Ok((
        rest.trim_end(),
        header.with_checkboxes(checkboxes),
    ))
}

fn parse_task_header(input: &str) -> IResult<&str, TaskHeader> {
    let (rest, idx) = parse_index(input)?;

    let (rest, is_checked) = parse_checkmark(rest)?;

    let (rest, name) = take_till(|ch| ch == '[')(rest)?;

    let (rest, priority) = parse_priority(rest)?;

    let header = TaskHeader {
        idx,
        is_checked,
        name: name.trim(),
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
    let input = input.trim_start();

    let is_digit = |chr: char| {
        let chr = chr as u8;
        (0x30..=0x39).contains(&chr)
    };

    let (rest, digits) = take_while(is_digit)(input)?;

    // Ensure digits is not empty
    let (_, _): (&str, &str) = take(1_usize)(digits)?;

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

pub struct Parser;

impl Parser {
    pub fn parse(input: &str) -> Result<TaskSet> {
        // This function reimplements nom's many0 because it
        // somehow behaves incorrectly here
        let mut tasks = Vec::new();
        let mut rest = input;

        loop {
            match parse_task(rest) {
                Ok((new_rest, task)) => {
                    tasks.push(task);
                    rest = new_rest;
                }
                Err(err) => {
                    eprintln!("Parsing problem: {err}");
                    // TODO: check if this problem came up
                    // because of an empty string (which is
                    // expected) or if because of a more severe
                    // error
                    break;
                }
            }
        }

        Ok(TaskSet(tasks))
    }
}

#[cfg(test)]
mod tests {
    use dodo_internals::{
        utils::today, Checkbox, Priority, Task, TaskSet,
    };

    use super::{
        parse_checkmark, parse_index, parse_task_header, Parser,
    };
    use crate::parser::{
        parse_checkbox, parse_priority, parse_task, TaskHeader,
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
    fn parse_task_0() {
        let task = "1. [ ] Fill out my tasks [HIGH]\n";

        assert_eq!(
            parse_task(task),
            Ok((
                "\n",
                Task {
                    name: "Fill out my tasks".into(),
                    is_done: false,
                    creation_date: today(),
                    due_date: None,
                    priority: Priority::High,
                    checklist: [].into_iter().collect()
                }
            ))
        );
    }

    #[test]
    fn parse_task_1() {
        let task = "1. [ ] Fill out my tasks [HIGH]\n  * [ ] Figure out how to use dodo\n";

        assert_eq!(
            parse_task(task),
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
            parse_task(task),
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
    fn parses_many_tasks() {
        let task = "1. [ ] Fill out my tasks [HIGH]\n  * [ ] Figure out how to use dodo\n* [x] Make this test pass\n2. [ ] Update taskset [HIGH]\n  * [ ] Do the dishes\n";

        assert_eq!(
            Parser::parse(task).unwrap(),
            TaskSet(vec![
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
                },
                Task {
                    name: "Update taskset".into(),
                    is_done: false,
                    creation_date: today(),
                    due_date: None,
                    priority: Priority::High,
                    checklist: [Checkbox::with_description(
                        "Do the dishes".into()
                    )]
                    .into_iter()
                    .collect()
                }
            ])
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
