# Agent Guidelines for Rust API

## Build/Test Commands
- `cargo build` - Build the project
- `cargo check` - Fast compile check without building
- `cargo test` - Run all tests
- `cargo test <test_name>` - Run specific test
- `cargo clippy` - Lint code (install with `rustup component add clippy`)
- `cargo fmt` - Format code (install with `rustup component add rustfmt`)

## Code Style
- Use `snake_case` for functions, variables, modules
- Use `PascalCase` for structs, enums, traits
- Use `SCREAMING_SNAKE_CASE` for constants
- Prefer explicit types over `auto`/inference when clarity helps
- Use `Result<T, E>` for error handling, avoid panics in production code
- Import organization: std library first, external crates, then local modules
- Use `#[derive(Debug)]` on structs/enums for debugging
- Prefer `&str` over `String` for function parameters when possible
- Use `anyhow::Result` for general error handling, custom errors for API responses

## Project Structure
- `/src/main.rs` - Entry point with Axum router setup
- `/src/models/` - Data structures and database models
- `/src/routes/` - HTTP route handlers
- `/src/services/` - Business logic layer