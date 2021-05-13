use nom::bits;
use nom::error::{ErrorKind, make_error};
use nom::IResult;
use nom::multi::count;
use nom::number::complete::be_u16;
use nom::sequence::tuple;

#[derive(Debug, PartialEq)]
pub struct Header {
    pub transaction_id: u16,
    pub flags: Flags,
    pub num_questions: u16,
    pub num_answers: u16,
    pub num_authorities: u16,
    pub num_additionals: u16,
}

impl Header {
    pub fn from_bytes(input: &[u8]) -> IResult<&[u8], Header> {
        let (input, (t_id, flags, header_fields)) =
            tuple((be_u16, Flags::from_bytes, count(be_u16, 4)))(input)?;
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
}

#[derive(Debug, PartialEq)]
pub struct Flags {
    pub message_type: QueryResponse,
    pub response_code: ResponseCode,
}

#[derive(Debug, PartialEq)]
pub enum QueryResponse {
    Query,
    Response,
}

#[derive(Debug, PartialEq)]
pub enum ResponseCode {
    NoError,
    FormatError,
    ServerError,
    NameError,
}

impl Flags {
    pub fn from_bytes(input: &[u8]) -> IResult<&[u8], Flags> {
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
}
