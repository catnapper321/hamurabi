use nom::{
    branch::alt,
    bytes::complete::tag_no_case,
    character::complete::{char, digit0, digit1, line_ending, not_line_ending, space0},
    combinator::{map, map_res, opt, recognize},
    sequence::{separated_pair, tuple},
};

pub fn parse_positive_i32_maybe(input: &str) -> Option<i32> {
    let v: nom::IResult<&str, i32> = map_res(digit1, |x: &str| x.parse::<i32>())(input);
    if let Ok((_, v)) = v {
        Some(v)
    } else {
        None
    }
}

pub enum WholeNumber {
    Number(i32),
    Default,
}

impl WholeNumber {
    pub fn parse_maybe(input: &str) -> Option<Self> {
        let mut p = alt((Self::parse_number, Self::parse_default));
        if let Ok((_, v)) = p(input) {
            Some(v)
        } else {
            None
        }
    }

    fn parse_number(input: &str) -> nom::IResult<&str, Self> {
        map(parse_i32, Self::Number)(input)
    }

    fn parse_default(input: &str) -> nom::IResult<&str, Self> {
        map(line_ending, |_| Self::Default)(input)
    }
}

fn parse_i32(input: &str) -> nom::IResult<&str, i32> {
    map_res(digit1, |x: &str| x.parse::<i32>())(input)
}

pub enum BuySell {
    Buy(i32),
    Sell(i32),
    Default,
}

impl BuySell {
    fn parse_buy(input: &str) -> nom::IResult<&str, Self> {
        let p = separated_pair(tag_no_case("buy"), space0, parse_i32);
        map(p, |(_, v)| Self::Buy(v))(input)
    }

    fn parse_sell(input: &str) -> nom::IResult<&str, Self> {
        let p = separated_pair(tag_no_case("sell"), space0, parse_i32);
        map(p, |(_, v)| Self::Sell(v))(input)
    }

    fn parse_default(input: &str) -> nom::IResult<&str, BuySell> {
        map(line_ending, |_| BuySell::Default)(input)
    }

    pub fn parse_maybe(input: &str) -> Option<Self> {
        let p = alt((Self::parse_buy, Self::parse_sell, Self::parse_default))(input);
        if let Ok((_, v)) = p {
            Some(v)
        } else {
            None
        }
    }
}
