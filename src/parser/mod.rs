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
        let mt = match qr {
            0 => Ok(QueryResponse::Query),
            1 => Ok(QueryResponse::Response),
            _ => Err(nom::Err::Error(make_error(i, ErrorKind::TagBits))),
        }?;
        let rc = match rc {
            0 => Ok(ResponseCode::NoError),
            1 => Ok(ResponseCode::FormatError),
            2 => Ok(ResponseCode::ServerError),
            3 => Ok(ResponseCode::NameError),
            _ => Err(nom::Err::Error(make_error(i, ErrorKind::TagBits))),
        }?;
        Ok((
            i,
            Flags {
                message_type: mt,
                response_code: rc,
            },
        ))
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

#[cfg(test)]
mod tests {
    use super::message::{Flags, Header, Message, Query, QueryResponse, ResponseCode};
    use super::parse_message;

    #[test]
    fn parses_dns_query() {
        let message = parse_message(get_valid_query()).unwrap();
        assert_eq!(message, get_expected_message())
    }

    #[test]
    fn fails_on_invalid_query() {
        let message = parse_message(get_invalid_query()).ok();
        assert_eq!(message, None)
    }

    fn get_expected_message() -> Message {
        Message {
            header: Header {
                transaction_id: 5538,
                flags: Flags {
                    message_type: QueryResponse::Query,
                    response_code: ResponseCode::NoError,
                },
                num_questions: 1,
                num_answers: 0,
                num_authorities: 0,
                num_additionals: 1,
            },
            queries: vec![Query {
                name: "www.bengesoff.uk.tld.com".to_string(),
                resource_type: 0,
                class: 256,
            }],
        }
    }

    fn get_valid_query() -> &'static [u8] {
        &[
            21, 162, 1, 32, 0, 1, 0, 0, 0, 0, 0, 1, 3, 119, 119, 119, 9, 98, 101, 110, 103, 101,
            115, 111, 102, 102, 2, 117, 107, 3, 116, 108, 100, 3, 99, 111, 109, 0, 0, 1, 0, 1, 0,
            0, 41, 16,
        ]
    }

    fn get_invalid_query() -> &'static [u8] {
        &[
            115, 111, 102, 102, 2, 117, 107, 3, 116, 108, 100, 3, 99, 111, 109, 0, 0, 1, 0, 1, 0,
            0, 41, 16,
        ]
    }
}
