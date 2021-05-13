use nom::error::{ErrorKind, make_error};
use nom::IResult;
use nom::multi::fold_many1;
use nom::multi::length_data;
use nom::number::complete::{be_u16, u8};
use nom::sequence::tuple;

#[derive(Debug, PartialEq)]
pub struct Query {
    pub name: String,
    pub resource_type: u16,
    pub class: u16,
}

impl Query {
    pub fn from_bytes(input: &[u8]) -> IResult<&[u8], Query> {
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
            let segment = std::str::from_utf8(chars).unwrap();
            Ok((input, segment))
        }
    }
}
