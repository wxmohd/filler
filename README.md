# Filler Game

A complete implementation of the Filler game in Rust with AI opponents and terminal-based gameplay.

## Features

- **Terminal Game Engine**: Standard Filler protocol implementation
- **AI Bot**: Multiple difficulty levels (Easy, Medium, Hard, Expert)
- **Human vs AI**: Interactive gameplay with input validation
- **AI vs AI**: Watch bots compete against each other
- **Visualizer**: Clean terminal-based game display
- **Docker Support**: Containerized environment for testing

## Building

```bash
# Build all binaries
cargo build --release

# Run tests
cargo test
```

## Usage

### Human vs AI
```bash
cargo run --bin game_engine -- -h
```

### AI vs AI
```bash
cargo run --bin game_engine -- -p1 ./solution/filler_ai -p2 ./solution/filler_ai
```

### Bot vs Bot with Custom Players
```bash
cargo run --bin game_engine -- -p1 robots/bender -p2 robots/terminator
```

### Using Maps
```bash
cargo run --bin game_engine -- -f maps/map01 -p1 ./solution/filler_ai -p2 robots/wall_e
```

## Docker

```bash
# Build container
docker build -t filler .

# Run container
docker run -it filler

# Test with audit commands
./game_engine -f maps/map01 -p1 robots/bender -p2 robots/terminator
```

## Game Rules

- Players start at opposite corners of the board
- Each turn, place a piece with exactly one cell overlapping your territory
- Cannot overlap opponent territory
- Win by controlling the most area when no valid moves remain

## AI Difficulty Levels

- **Easy**: Random valid moves
- **Medium**: Greedy territory expansion
- **Hard**: Minimax with alpha-beta pruning (depth 3)
- **Expert**: Advanced minimax with optimizations (depth 5)

## Files Structure

- `src/bin/game_engine.rs` - Main game engine
- `src/bin/filler_ai.rs` - AI bot executable
- `src/game.rs` - Core game logic
- `src/ai.rs` - AI implementations
- `src/piece.rs` - Piece generation
- `maps/` - Sample map files
- `robots/` - Test opponent bots
