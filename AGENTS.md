# AI Agent Guide for StockTalkerCLI (stocktalkerai)

Welcome, AI Agent! You are looking at `stocktalkerai-cli`, a Rust-based, agent-native CLI for the StockTalker AI backend.
This file provides instructions on how to install, configure, and use the CLI to interact with our REST API.

## Installation 

Since you are an agent, you can clone this repository and build it locally on your VM:
```bash
# Clone the repository
git clone https://github.com/tgel0/stocktalkerai-cli
cd stocktalkerai-cli

# Install Rust if you don't have it
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
source "$HOME/.cargo/env"

# Build the CLI
cargo build --release
```
Your compiled binary will be located at `./target/release/stocktalkerai`.

## Rules for Agents

1. **Always use `--output json` or `-o json`**: While the CLI defaults to human-readable tables, you must append `-o json` to all requests to parse the standard JSON envelope (`{"data": ... }`).
2. **Handle the Envelope**: All JSON responses are wrapped in a `data` key. Extract the array or object from `res.data`.
3. **Authentication**: Some commands require an API key. 
   - Export it in your shell environment: `export STOCKTALKERAI_API_KEY="sk_..."`
   - Or pass it directly via `--api-key sk_...`

## Command Directory

### Public Routes (No Auth Required)
* **Status**: `stocktalkerai status -o json` -> Healthcheck the API.
* **Price**: `stocktalkerai price <ticker> -o json` -> Current price.
* **History**: `stocktalkerai price <ticker> --history -o json` -> 30-day historical data.
* **Indicators**: `stocktalkerai indicators <ticker> -o json` -> Calculates SMA and RSI.

### Authenticated Routes (Requires STOCKTALKERAI_API_KEY)
* **Account**: `stocktalkerai account -o json` -> Returns your user tier and active alert count.
* **Alert Ls**: `stocktalkerai alert ls [--list <id>] -o json` -> List all alerts, optionally filtered by a list ID.
* **Alert Create**: `stocktalkerai alert create "Notify me when AAPL crosses $200" [--list <id>] [--note <"text">] [--pin] [--color blue] [--full] -o json` -> Generates an alert, optionally attaching it to a list, adding a note, pinning, or assigning a color.
* **Alert Edit**: `stocktalkerai alert edit <id> [--list <id>] [--note <"text">] [--pin] [--unpin] [--color blue] [--clear-color] -o json` -> Updates an existing alert. Pass `--pin`/`--unpin` to toggle pin status, `--color <name>` to set a color, `--clear-color` to remove color. Pass an empty string (`""`) to remove a list or note.
* **Alert Archive**: `stocktalkerai alert archive <id> -o json` -> Safely archives (disables) an alert.
* **List Ls**: `stocktalkerai list ls -o json` -> Retrieves all alert list groups assigned to the user.

See `agents/tool-catalog.json` for exact JSON schemas mapping each command for OpenClaw.
