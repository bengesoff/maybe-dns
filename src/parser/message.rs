#[derive(Debug)]
pub struct Message {
    pub header: Header,
    pub queries: Vec<Query>,
}

#[derive(Debug)]
pub struct Header {
    pub transaction_id: u16,
    pub flags: Flags,
    pub num_questions: u16,
    pub num_answers: u16,
    pub num_authorities: u16,
    pub num_additionals: u16,
}

#[derive(Debug)]
pub struct Query {
    pub name: String,
    pub resource_type: u16,
    pub class: u16,
}

#[derive(Debug)]
pub struct Flags {
    pub message_type: QueryResponse,
    pub response_code: ResponseCode,
}

#[derive(Debug)]
pub enum QueryResponse {
    Query,
    Response,
}

#[derive(Debug)]
pub enum ResponseCode {
    NoError,
    FormatError,
    ServerError,
    NameError,
}
