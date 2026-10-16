use padi_core::prelude::*;

mod controllers;
mod models;
mod resources;
mod routes;

#[tokio::main]
async fn main() -> Result<(), ApiError> {
    let root = env!("CARGO_MANIFEST_DIR");
    let args: Vec<String> = std::env::args().collect();
    let command = args.get(1).map(|s| s.as_str());

    match command {
        Some("serve") | None => {
            // Apply optional server flags: --port=<port> --host=<host>
            for arg in args.iter().skip(1) {
                if let Some(port) = arg.strip_prefix("--port=") {
                    std::env::set_var("APP_PORT", port);
                } else if let Some(host) = arg.strip_prefix("--host=") {
                    std::env::set_var("APP_HOST", host);
                }
            }
            Application::new(root, routes::api()).run().await
        }
        Some(_) => {
            padi_core::env::load(&format!("{root}/.env"));
            let _ = Database::connect().await;
            Console::new(root).run(&args[1..]).await
        }
    }
}
