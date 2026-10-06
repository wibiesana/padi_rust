use padi_core::{migrator::Migrator, prelude::*};

mod controllers;
mod models;
mod resources;
mod routes;

#[tokio::main]
async fn main() -> Result<(), ApiError> {
    let root = env!("CARGO_MANIFEST_DIR");
    let args: Vec<String> = std::env::args().collect();

    // CLI: `cargo run -- migrate` | `cargo run -- migrate:rollback`
    if let Some(cmd) = args.get(1) {
        padi_core::env::load(&format!("{root}/.env"));
        Database::connect().await?;
        let dir = format!("{root}/database/migrations");
        match cmd.as_str() {
            "migrate" => println!("Migrated: {:?}", Migrator::run(&dir).await?),
            "migrate:rollback" => println!("Rolled back: {:?}", Migrator::rollback(&dir).await?),
            other => eprintln!("Unknown command: {other}"),
        }
        return Ok(());
    }

    Application::new(root, routes::api()).run().await
}
