use std::str;

use nom::error::{make_error, ErrorKind};
use nom::multi::{count, fold_many1, length_data};
use nom::number::complete::{be_u16, u8};
use nom::sequence::tuple;
use nom::IResult;

use errors::ParseError;
use message::{Header, Message, Query};

pub mod errors;
pub mod message;

extern crate nom;

pub fn parse_message(input: &[u8]) -> Result<Message, ParseError> {
    match message(input).ok() {
        None => Err(ParseError),
        Some((_, msg)) => Ok(msg),
    }
}

fn message(input: &[u8]) -> IResult<&[u8], Message> {
    let (input, header) = header(input)?;
    let (input, queries) = count(query, header.num_questions as usize)(input)?;
    Ok((input, Message { header, queries }))
}

fn header(input: &[u8]) -> IResult<&[u8], Header> {
    let (input, header_fields) = count(be_u16, 6)(input)?;
    Ok((
        input,
        Header {
            transaction_id: header_fields[0],
            // TODO: flags: header_fields[1],
            num_questions: header_fields[2],
            num_answers: header_fields[3],
            num_authorities: header_fields[4],
            num_additionals: header_fields[5],
        },
    ))
}

fn query(input: &[u8]) -> IResult<&[u8], Query> {
    let (input, (name, resource_type, class)) = tuple((name, be_u16, be_u16))(input)?;
    Ok((
        input,
        Query {
            name: name.join("."),
            resource_type,
            class,
        },
    ))
}

fn name(input: &[u8]) -> IResult<&[u8], Vec<&str>> {
    fold_many1(name_segment, Vec::new(), |mut acc, item| {
        acc.push(item);
        acc
    })(input)
}

fn name_segment(input: &[u8]) -> IResult<&[u8], &str> {
    let (input, chars) = length_data(u8)(input)?;
    match chars.len() {
        0 => Err(nom::Err::Error(make_error(input, ErrorKind::Count))),
        _ => {
            let segment = str::from_utf8(chars).unwrap();
            Ok((input, segment))
        }
    }
}
