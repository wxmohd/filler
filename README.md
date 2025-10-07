# Filler Game

A terminal-based Filler AI bot in Rust that speaks the standard game protocol (stdin/stdout) and can be run against the official engines and bots provided in `docker_image/`.

## Features

- **AI Bot**: Protocol-compliant standalone bot (`filler_ai`) at `solution/filler_ai.rs`
- **Strategy Engine**: Heuristic-based placement with legality checks and territory awareness
- **Docker Support**: Uses the existing `docker_image/` folder with pre-built game engines and opponent bots
- **Cross-platform workflow**: Build your bot natively; run full matches inside Docker or WSL

## Quick Start

### Build (Release)

```bash
cargo build --release
```

### Run the AI bot locally

```bash
# Runs and waits for engine-formatted input on stdin
./target/release/filler_ai

# OR with an input file
./target/release/filler_ai < test_input.txt
```

## Docker Usage

The repo integrates with the existing `docker_image/` folder containing pre-built opponent bots and engines.

### Build Docker image
```bash
docker build -t filler .
```

### Run container (mount your bot)

- PowerShell (Windows):
```powershell
docker run -v "${PWD}\solution:/filler/solution" -it filler
```

- Git Bash / macOS / Linux:
```bash
docker run -v "$(pwd)/solution":/filler/solution -it filler
```

### Play matches against bots (inside container)
```bash
# Use the provided Linux engine with official maps and bots
./linux_game_engine -f maps/map01 -p1 solution/filler_ai -p2 linux_robots/bender
./linux_game_engine -f maps/map01 -p1 solution/filler_ai -p2 linux_robots/h2_d2
./linux_game_engine -f maps/map01 -p1 solution/filler_ai -p2 linux_robots/wall_e

# For Apple Silicon (if using the macOS/ARM container), use the M1 engine and bots
./m1_game_engine -f maps/map01 -p1 solution/filler_ai -p2 m1_robots/bender
```

## Available Opponent Bots (from docker_image/)

- **bender**
- **h2_d2**  
- **wall_e**
- **terminator** (very strong)

## Game Rules (engine-enforced)

1. Players start at opposite corners of the board (`@` for Player 1, `$` for Player 2)
2. Each turn, players receive a piece
3. A legal placement must overlap your existing territory by exactly one cell and not collide with the opponent

## Maps

The project includes three official maps (via `docker_image/maps/`):

- **map00**: 20x15 grid
- **map01**: 40x24 grid  
- **map02**: 100x100 grid

## Testing / Validation

- There is no Rust test suite included in this repo. Validate behavior by running matches inside Docker (or WSL) using the engines and bots above.
- You can also pipe recorded engine input into `filler_ai` locally for quick checks.

## Performance & Compatibility

- The bot is protocol-compliant (reads from stdin, writes `"x y"` to stdout per move)
- Designed to work with the provided engines and maps inside Docker/WSL
- Windows users: build and run `filler_ai` natively; use Docker or WSL for full matches with the official engines/bots

## Usage Examples

```bash
# Quick test against bender (inside container)
./linux_game_engine -f maps/map01 -p1 solution/filler_ai -p2 linux_robots/bender

# Quiet mode (if supported by the engine)
./linux_game_engine -f maps/map00 -p1 solution/filler_ai -p2 linux_robots/wall_e -q

# With custom seed for reproducibility (if supported)
./linux_game_engine -f maps/map02 -p1 solution/filler_ai -p2 linux_robots/h2_d2 -s 12345

# Throttled mode for visualization (if supported)
./linux_game_engine -f maps/map01 -p1 solution/filler_ai -p2 linux_robots/bender -r
```

## Dependencies

- `rand = "0.8"` - Random number generation for internal heuristics
