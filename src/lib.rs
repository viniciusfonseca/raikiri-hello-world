use waki::{handler, ErrorCode, Request, Response};

#[handler]
fn h(r: Request) -> Result<Response, ErrorCode> { http_handler(r) }

pub fn http_handler(_req: Request) -> Result<Response, ErrorCode> {
    println!("Data from stdout");
    Response::builder()
        .body(format!("Hello World!"))
        .build()
}

#[cfg(test)]
mod tests {
    use waki::Request;
    use waki::Method::*;
    use crate::http_handler;

    #[test]
    fn test_hello() {
        let req = Request::builder(Get, "http://localhost:8080").build().unwrap();
        let res = http_handler(req).unwrap();
        assert_eq!(res.status_code(), 200);
        assert_eq!(res.body().unwrap(), b"Hello World!");
    }
}