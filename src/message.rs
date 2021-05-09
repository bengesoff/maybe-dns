pub fn parse(message: &[u8]) -> Message {
    let header = parse_header(&message[0..12]);
    let queries = parse_queries(header.num_questions, &message[12..]);
    Message {
        header: header,
        queries: queries,
    }
}

fn parse_header(message: &[u8]) -> Header {
    Header {
        transaction_id: byte_pair_to_u16(message[0], message[1]),
        num_questions: byte_pair_to_u16(message[4], message[5]),
    }
}

fn parse_queries(num_queries: u16, message: &[u8]) -> Vec<Query> {
    let mut cursor = 0;
    let mut queries: Vec<Query> = Vec::new();

    for _n in 0..num_queries {
        let mut name_segments: Vec<&str> = Vec::new();
        
        loop {
            let segment_length = message[cursor] as usize;

            cursor += 1;

            if segment_length == 0 {
                break;
            }

            let segment = std::str::from_utf8(&message[cursor..cursor+segment_length]).unwrap();
            name_segments.push(segment);

            cursor += segment_length;
        }

        let name = name_segments.join(".");

        queries.push(Query {
            name: name,
            resource_type: byte_pair_to_u16(message[cursor], message[cursor+1]),
            class: byte_pair_to_u16(message[cursor+2], message[cursor+3]),
        });

        cursor += 4;
    }

    queries
}

fn byte_pair_to_u16(a: u8, b: u8) -> u16 {
    ((a as u16) << 8) + (b as u16)
}

#[derive(Debug)]
pub struct Message {
    header: Header,
    queries: Vec<Query>,
}

#[derive(Debug)]
struct Header {
    transaction_id: u16,
    //flags: Flags,
    num_questions: u16,
//    num_answers: u16,
    //num_authorities: u16,
    //num_additionals: u16,
}

#[derive(Debug)]
struct Query {
    name: String,
    resource_type: u16,
    class: u16,
}

struct Flags {
    message_type: QueryResponse,
    response_code: Error
}

enum QueryResponse {
    Query,
    Response,
}

enum Error {
    NoError,
    NotFound,
}
