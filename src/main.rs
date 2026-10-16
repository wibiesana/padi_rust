use padi_core::prelude::*;

mod controllers;
mod models;
mod resources;
mod routes;

#[tokio::main]
async fn main() -> Result<(), ApiError> {
    let root = env!("CARGO_MANIFEST_DIR");
    let args: Vec<String> = std::env::args().collect();

    // CLI handling
    if args.len() > 1 && args[1] != "serve" {
        padi_core::env::load(&format!("{root}/.env"));
        let _ = Database::connect().await;
        let console = Console::new(root);
        console.run(&args[1..]).await?;
        return Ok(());
    }

    if args.len() > 1 && args[1] == "serve" {
        for arg in &args[2..] {
            if let Some(port) = arg.strip_prefix("--port=") {
                std::env::set_var("APP_PORT", port);
            } else if let Some(host) = arg.strip_prefix("--host=") {
                std::env::set_var("APP_HOST", host);
            }
        }
    }

    Application::new(root, routes::api()).run().await
}
