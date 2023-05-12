use nom::{
    IResult,
    error::{VerboseError, context},
    character::complete::{char, digit1},
    bytes::complete::{take_till, tag},
    sequence::{tuple, pair, delimited, separated_pair, preceded},
    branch::alt,
    combinator::{opt, recognize, map},
    character::complete::{alpha1, alphanumeric1, multispace0},
    multi::many0
};

use crate::ast::{Kvp, Literal, LiteralKind, Vec3};

pub fn string(input: &str) -> IResult<&str, &str, VerboseError<&str>> {
    context(
        "string",
        alt((
            delimited(
                char('\''),
                take_till(|c| c == '\''),
                char('\'')
            ),
            delimited(
                char('"'),
                take_till(|c| c == '"'),
                char('"')
            )
        ))
    )(input)
}

pub fn number(input: &str) -> IResult<&str, &str, VerboseError<&str>> {
    context(
        "number",
        recognize(
            tuple((
                digit1,
                opt(
                    tuple((
                        char('.'),
                        digit1
                    ))
                ),
                opt(
                    tuple((
                        alt((
                            char('e'),
                            char('E')
                        )),
                        digit1
                    ))
                )
            ))
        )
    )(input)
}

pub fn boolean(input: &str) -> IResult<&str, &str, VerboseError<&str>> {
    context(
        "boolean",
        alt((
            tag("true"),
            tag("false")
        ))
    )(input)
}

pub fn literal(input: &str) -> IResult<&str, Literal, VerboseError<&str>> {
    context(
        "literal",
        alt((
            map(
                string,
                |value| Literal { kind: LiteralKind::String, value }
            ),
            map(
                number,
                |value| Literal { kind: LiteralKind::Number, value }
            ),
            map(
                boolean,
                |value| Literal { kind: LiteralKind::Boolean, value }
            ),
        ))
    )(input)
}

pub fn identifier(input: &str) -> IResult<&str, &str, VerboseError<&str>> {
    context(
        "identifier",
        recognize(
            pair(
                alt((
                    tag("_"), 
                    alpha1
                )),
                many0(
                    alt((
                        tag("_"),
                        alphanumeric1
                    ))
                )
            )
        )
    )(input)
}

pub fn kvp(input: &str) -> IResult<&str, Kvp, VerboseError<&str>> {
    context(
        "kvp",
        map(
            separated_pair(
                identifier,
                delimited(
                    multispace0,
                    char(':'),
                    multispace0
                ),
                literal
            ),
            |(key, value)| Kvp { key, value }
        )
    )(input)
}

pub fn vec3(input: &str) -> IResult<&str, Vec3, VerboseError<&str>> {
    context(
        "vec3",
        map(
            tuple((
                delimited(
                    multispace0,
                    number,
                    multispace0
                ),
                preceded(
                    char(','),
                    delimited(
                        multispace0,
                        number,
                        multispace0
                    )
                ),
                preceded(
                    char(','),
                    delimited(
                        multispace0,
                        number,
                        multispace0
                    )
                )
            )),
            |(x, y, z)| Vec3 { x, y, z }
        )
    )(input)
}

#[test]
fn test() {
    assert_eq!(string("'hey'--"), Ok(("--", "hey")));
    assert_eq!(number("23.5e10--"), Ok(("--", "23.5e10")));
    assert_eq!(boolean("true--"), Ok(("--", "true")));
    assert_eq!(identifier("ab123--"), Ok(("--", "ab123")));

    assert_eq!(
        kvp("str: 'hello'--"), 
        Ok((
            "--", 
            Kvp { 
                key: "str", 
                value: Literal { 
                    kind: LiteralKind::String, 
                    value: "hello" 
                } 
            }
        ))
    );

    assert_eq!(
        vec3("2.5, 3, 10e5--"),
        Ok((
            "--",
            Vec3 {
                x: "2.5",
                y: "3",
                z: "10e5"
            }
        ))
    )
}