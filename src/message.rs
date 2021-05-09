#[derive(Debug)]
pub struct Message {
    pub header: Header,
    pub queries: Vec<Query>,
}

#[derive(Debug)]
pub struct Header {
    pub transaction_id: u16,
    //flags: Flags,
    pub num_questions: u16,
    //num_answers: u16,
    //num_authorities: u16,
    //num_additionals: u16,
}

#[derive(Debug)]
pub struct Query {
    pub name: String,
    pub resource_type: u16,
    pub class: u16,
}

struct Flags {
    message_type: QueryResponse,
    response_code: Error,
}

enum QueryResponse {
    Query,
    Response,
}

enum Error {
    NoError,
    NotFound,
}
