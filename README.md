# Rice CLI Setup Tool

A command-line interface for setting up [Rice](https://github.com/console-rs/rice) in your JavaScript/TypeScript projects.

This tool helps you:

- Configure Rice Storage and State services.
- Generate `rice.config.js`.
- Populate environment variables in `.env`.
- Verify connectivity to your Rice instance.

## Installation

### Quick Install (macOS / Linux)

To install the latest version automatically:

```bash
curl -fsSL https://raw.githubusercontent.com/shankha98/rice-cli/main/install.sh | bash
```

This will install the `rice` command to `/usr/local/bin`.

### Manual Installation

Download the pre-built binary for your platform from the [Releases page](https://github.com/shankha98/rice-cli/releases).

### Building from Source

To build from source, ensure you have Rust installed:

```bash
cargo install --path .
```

## Usage

Run the CLI in the root of your project:

```bash
# Setup Rice (Interactive) - Default
cargo run -- setup
# OR
cargo run

# Show current configuration
cargo run -- config

# Check connection to Rice instance
cargo run -- check

# Show help
cargo run -- --help
```

### Setup Command

The setup command (`setup` or default) will guide you through:

1. Enable/Disable Storage and State services.
2. Provide connection details (URL, Auth Token, etc.).
3. Generate `rice.config.js` and update `.env`.
4. Verify connection to the Rice instance.

### Config Command

The `config` command reads `.env` and `rice.config.js` in the current directory and displays the configured values (masking sensitive tokens).

### Check Command

The `check` command uses the configured values to attempt a connection to the Rice instance health endpoint.

## Development

- `make build`: Build the project.
- `make run`: Run the CLI.
- `make test`: Run unit tests.
- `make integration-test`: Run integration tests using a temporary project directory.

## License

Proprietary. All Rights Reserved.
