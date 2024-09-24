wit_bindgen::generate!({
    path: "wit",
    world: "http",
    pub_export_macro: true,
    generate_all,
});

use exports::wasi::http::incoming_handler::Guest;
use wasi::http::types::{self, Headers, IncomingRequest, OutgoingBody, OutgoingResponse, ResponseOutparam};

fn handler(_req: IncomingRequest) -> Result<OutgoingResponse, types::ErrorCode> {
    println!("Logging to STDOUT");
    let body = "Hello World";
    let response = OutgoingResponse::new(Headers::new());
    let outgoing_body = response.body().unwrap().write().unwrap();
    outgoing_body.blocking_write_and_flush(body.as_bytes()).unwrap();
    OutgoingBody::finish(response.body().unwrap(), None).unwrap();
    Ok(response)
}

pub fn handle_response(response_out: ResponseOutparam, response: OutgoingResponse) {
    let outgoing_response = OutgoingResponse::new(response.headers().try_into().unwrap());
    outgoing_response
        .set_status_code(response.status_code())
        .unwrap();
    ResponseOutparam::set(response_out, Ok(outgoing_response));
}

struct Component;

impl Guest for Component {

    fn handle(request: IncomingRequest, response_out: ResponseOutparam) {
        match handler(request) {
            Ok(resp) => handle_response(response_out, resp),
            Err(e) => ResponseOutparam::set(response_out, Err(e)),
        }
    }
}

export!(Component);
