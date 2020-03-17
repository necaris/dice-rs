use nom::bytes::complete::tag_no_case;
use nom::character::complete::{alpha1, digit1, space0, space1};
use nom::combinator::opt;
use nom::multi::many1;
use nom::sequence::tuple;
use nom::IResult;

use crate::die::*;

// NOTE: this is pretty much copied wholesale from the nom tutorial
#[derive(Default)]
pub struct ParseError;
impl std::fmt::Display for ParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "A parsing error occurred.")
    }
}
impl std::fmt::Debug for ParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        <ParseError as std::fmt::Display>::fmt(self, f)
    }
}
impl std::error::Error for ParseError {}

fn parse_dice(i: &str) -> IResult<&str, Vec<Die>> {
    let (remaining, (count_spec, die)) = tuple((opt(digit1), parse_sides))(i)?;
    let count = match count_spec {
        Some(digits) => {
            let parsed = digits.parse::<u8>();
            if parsed.is_err() {
                return Err(nom::Err::Error((digits, nom::error::ErrorKind::TooLarge)));
            }
            parsed.unwrap()
        }
        None => 1,
    };
    let mut dice: Vec<Die> = Vec::new();
    for _ in 0..count {
        dice.push(die.clone())
    }
    Ok((remaining, dice))
}

fn parse_sides(i: &str) -> IResult<&str, Die> {
    let (remaining, (_tag, side_spec)) = tuple((tag_no_case("d"), digit1))(i)?;
    let sides = side_spec.parse::<u8>();
    // It _should_ be parseable, because it's a digit sequence, but
    // it might just be too big to be reasonable
    if sides.is_err() {
        return Err(nom::Err::Error((
            side_spec,
            nom::error::ErrorKind::TooLarge,
        )));
    }
    Ok((
        remaining,
        Die {
            sides: sides.unwrap(),
        },
    ))
}

fn parse_comparison(i: &str) -> IResult<&str, &str> {
    let (remaining, (_tag, _space, name)) = tuple((tag_no_case("vs"), space1, alpha1))(i)?;
    Ok((remaining, name))
}

fn parse_spec(i: &str) -> IResult<&str, DiceSpec> {
    let (remaining, dice_triples) = many1(tuple((space0, parse_dice, space0)))(i)?;
    let (remaining, comparison) = opt(parse_comparison)(remaining)?;
    let dice: Vec<Die> = dice_triples
        .iter()
        .map(|(_, dice, _)| dice)
        .cloned()
        .flatten()
        .collect();
    Ok((
        remaining,
        DiceSpec {
            dice,
            compare_to: comparison,
        },
    ))
}

pub fn parse_command<'a>(cmd: &'a str) -> Result<DiceSpec<'a>, ParseError> {
    let result = parse_spec(cmd.trim());
    match result {
        Ok((_, spec)) => Ok(spec),
        Err(_) => Err(ParseError {}),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_sides() {
        assert_eq!(parse_sides("d6"), Ok(("", Die { sides: 6 })));
        assert_eq!(
            parse_sides("potato"),
            Err(nom::Err::Error(("potato", nom::error::ErrorKind::Tag)))
        );
        assert_eq!(
            parse_sides("doo"),
            Err(nom::Err::Error(("oo", nom::error::ErrorKind::Digit)))
        );
        assert_eq!(
            parse_sides("d999"),
            Err(nom::Err::Error(("999", nom::error::ErrorKind::TooLarge)))
        );
    }

    #[test]
    fn test_parse_die() {
        assert_eq!(
            parse_dice("3d6"),
            Ok((
                "",
                vec![Die { sides: 6 }, Die { sides: 6 }, Die { sides: 6 },]
            ))
        );
    }

    #[test]
    fn test_parse_command() {
        let sides = DiceSpec {
            dice: vec![Die { sides: 6 }],
            compare_to: None,
        };
        assert_eq!(parse_command("d6").unwrap(), sides);
        let number_sides = DiceSpec {
            dice: vec![Die { sides: 6 }, Die { sides: 6 }, Die { sides: 6 }],
            compare_to: None,
        };
        assert_eq!(parse_command("3d6").unwrap(), number_sides);
        let lots = DiceSpec {
            dice: vec![
                Die { sides: 6 },
                Die { sides: 6 },
                Die { sides: 12 },
                Die { sides: 12 },
                Die { sides: 12 },
                Die { sides: 20 },
            ],
            compare_to: None,
        };
        assert_eq!(parse_command("2d6 3D12 1d20").unwrap(), lots);
        let comparison = DiceSpec {
            dice: vec![Die { sides: 20 }, Die { sides: 20 }],
            compare_to: Some("charisma"),
        };
        assert_eq!(parse_command("2d20 vs charisma").unwrap(), comparison);
    }
}
