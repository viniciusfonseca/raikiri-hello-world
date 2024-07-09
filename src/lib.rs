raikiri_bindings::init!();

async fn handler(_req: Request) -> ModuleResponse {
    println!("Logging to STDOUT");
    let body = "Hello World";
    ModuleResponseBuilder::new()
        .status(200)
        .body(body.as_bytes().to_vec())
        .finish()
}
