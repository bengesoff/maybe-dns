use nom::multi::count;

use super::errors::ParseError;
use super::header::Header;
use super::query::Query;

#[derive(Debug, PartialEq)]
pub struct Message {
    pub header: Header,
    pub queries: Vec<Query>,
}

impl Message {
    pub fn from_bytes(input: &[u8]) -> Result<Message, ParseError> {
        let (input, header) = Header::from_bytes(input).map_err(|_| ParseError)?;
        let (_input, queries) = count(Query::from_bytes, header.num_questions as usize)(input)
            .map_err(|_| ParseError)?;
        Ok(Message { header, queries })
    }
}

#[cfg(test)]
mod tests {
    use super::Message;
    use super::super::header::{Flags, Header, QueryResponse, ResponseCode};
    use super::super::query::Query;

    #[test]
    fn parses_dns_query() {
        let message = Message::from_bytes(get_valid_query()).unwrap();
        assert_eq!(message, get_expected_message())
    }

    #[test]
    fn fails_on_invalid_query() {
        let message = Message::from_bytes(get_invalid_query()).ok();
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
