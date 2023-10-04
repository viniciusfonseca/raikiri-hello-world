raikiri_bindings::init!();

async fn handler(_event: String) -> ModuleResponse {
    let body = "Hello World".to_string();
    ModuleResponseBuilder::new()
        .status(200)
        .body(body)
        .finish()
}
