# CLI Starter Project

A comprehensive CLI application starter template built with Rust, featuring subcommands, error handling, logging, and colored output.

## Features

- 🚀 **Modern CLI Framework**: Built with `clap` for powerful argument parsing
- 🎨 **Colored Output**: Beautiful terminal output with `colored` crate
- 📝 **Structured Logging**: Comprehensive logging with `env_logger` and `log`
- ⚡ **Async Support**: Built-in async/await support with Tokio
- 🛡️ **Error Handling**: Robust error handling with `thiserror`
- 🧪 **Testing**: Ready-to-use testing setup with `assert_cmd`

## Installation

### Prerequisites

- Rust 1.70+ (for async traits support)
- Cargo

### Build from Source

```bash
# Clone or navigate to the project directory
cd cli-starter-project

# Build the project
cargo build --release

# Install globally (optional)
cargo install --path .
```

## Usage

### Basic Commands

```bash
# Show help
./target/release/cli-starter --help

# Say hello (defaults to "World")
./target/release/cli-starter hello

# Say hello to a specific person
./target/release/cli-starter hello --name "Alice"

# Show version information
./target/release/cli-starter version

# Enable verbose logging
./target/release/cli-starter --verbose hello
```

### Development

```bash
# Run in development mode
cargo run

# Run with specific command
cargo run -- hello --name "Bob"

# Run tests
cargo test

# Check code formatting
cargo fmt

# Run clippy linter
cargo clippy
```

## Project Structure

```
cli-starter-project/
├── Cargo.toml          # Project dependencies and metadata
├── README.md           # This file
└── src/
    ├── main.rs         # Application entry point
    ├── cli.rs          # CLI argument definitions
    ├── error.rs        # Custom error types
    └── commands/       # Command implementations
        ├── mod.rs      # Command module exports
        ├── hello.rs    # Hello command
        └── version.rs  # Version command
```

## Adding New Commands

1. **Add the command to `src/cli.rs`**:

   ```rust
   #[derive(Subcommand)]
   pub enum Commands {
       // ... existing commands ...
       NewCommand {
           #[arg(short, long)]
           option: String,
       },
   }
   ```

2. **Create the command module** in `src/commands/new_command.rs`:

   ```rust
   use colored::*;
   use log::info;
   use crate::error::AppError;

   pub async fn run(option: String) -> Result<(), AppError> {
       info!("Running new command with option: {}", option);
       println!("{}", "New command executed!".green().bold());
       Ok(())
   }
   ```

3. **Export the module** in `src/commands/mod.rs`:

   ```rust
   pub mod new_command;
   pub use new_command::*;
   ```

4. **Add the command handler** in `src/main.rs`:
   ```rust
   match cli.command {
       // ... existing matches ...
       Some(commands::Commands::NewCommand { option }) => {
           new_command::run(option).await?;
       }
   }
   ```

## Configuration

The project includes optional dependencies for configuration management:

- `serde` and `serde_json` for JSON configuration files
- `toml` for TOML configuration files

To enable these features, uncomment the relevant dependencies in `Cargo.toml`.

## Testing

The project includes integration testing setup:

```bash
# Run all tests
cargo test

# Run specific test
cargo test test_hello_command

# Run tests with output
cargo test -- --nocapture
```

## Contributing

1. Fork the repository
2. Create a feature branch
3. Make your changes
4. Add tests for new functionality
5. Run the test suite
6. Submit a pull request

## License

This project is licensed under the MIT License - see the LICENSE file for details.

## Acknowledgments

- [clap](https://github.com/clap-rs/clap) - Command line argument parsing
- [colored](https://github.com/mackwic/colored) - Terminal colors
- [tokio](https://tokio.rs/) - Async runtime
- [thiserror](https://github.com/dtolnay/thiserror) - Error handling
