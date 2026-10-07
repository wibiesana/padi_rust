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
    if args.len() > 1 {
        padi_core::env::load(&format!("{root}/.env"));
        let _ = Database::connect().await;
        let console = Console::new(root);
        console.run(&args[1..]).await?;
        return Ok(());
    }

    Application::new(root, routes::api()).run().await
}
