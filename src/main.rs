use std::net::TcpListener;
use zero2prod::run;

// ! Attribute Macro: Basically generates another AST.
#[tokio::main]
async fn main() -> std::io::Result<()> {
    let listener = TcpListener::bind("127.0.0.1:8000").expect("Failed to bind to port.");
    run(listener)?.await
}
