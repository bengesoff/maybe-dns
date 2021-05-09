use std::str;

use nom::bits;
use nom::error::{make_error, ErrorKind};
use nom::multi::{count, fold_many1, length_data};
use nom::number::complete::{be_u16, u8};
use nom::sequence::tuple;
use nom::IResult;

use errors::ParseError;
use message::{Flags, Header, Message, Query, QueryResponse, ResponseCode};

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
    let (input, (t_id, flags, header_fields)) = tuple((be_u16, flags, count(be_u16, 4)))(input)?;
    Ok((
        input,
        Header {
            transaction_id: t_id,
            flags,
            num_questions: header_fields[0],
            num_answers: header_fields[1],
            num_authorities: header_fields[2],
            num_additionals: header_fields[3],
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

fn flags(input: &[u8]) -> IResult<&[u8], Flags> {
    bits::bits::<_, _, nom::error::Error<(&[u8], usize)>, _, _>(|i| {
        let (i, qr) = bits::complete::take::<_, u8, _, _>(1usize)(i)?;
        let (i, _) = bits::complete::take::<_, u8, _, _>(11usize)(i)?;
        let (i, rc) = bits::complete::take::<_, u8, _, _>(4usize)(i)?;
        Ok((i, Flags {
            message_type: match qr {
                0 => QueryResponse::Query,
                1 => QueryResponse::Response,
                _ => unreachable!()
            },
            response_code: match rc {
                0 => ResponseCode::NoError,
                1 => ResponseCode::FormatError,
                2 => ResponseCode::ServerError,
                3 => ResponseCode::NameError,
                _ => unimplemented!()
            },
        }))
    })(input)
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
