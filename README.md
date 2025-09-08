# Filler Game

A terminal-based implementation of the Filler game in Rust with AI opponents.

## Features

- **Game Engine**: Terminal-based Filler game engine (`filler_engine`)
- **AI Bot**: Standalone AI bot (`filler_ai`) compatible with standard Filler protocol
- **Multiple AI Difficulties**: Easy (Random), Medium (Greedy), Hard/Expert (Minimax with alpha-beta pruning)
- **Interactive Gameplay**: Human vs AI, AI vs AI, and Human vs Human modes
- **Game Visualization**: Terminal-based board display with animations
- **Docker Support**: Uses existing `docker_image/` folder with pre-built opponent bots

## Quick Start

### Building the Project

```bash
cargo build --release
```

### Running the Game

#### Terminal Game Engine
```bash
# Human vs AI
./target/release/filler_engine

# AI vs AI
./target/release/filler_engine --ai-vs-ai

# Custom board size
./target/release/filler_engine --width 20 --height 15
```

#### Standalone AI Bot
```bash
# Test AI with input file
./target/release/filler_ai < test_input.txt
```

## Docker Usage

The project integrates with the existing `docker_image/` folder containing pre-built opponent bots:

### Build Docker Image
```bash
docker build -t filler .
```

### Run Container
```bash
docker run -v "$(pwd)/solution":/filler/solution -it filler
```

### Test Against Bots (Inside Container)
```bash
# Test against different bots using the pre-built game engine
./linux_game_engine -f maps/map01 -p1 solution/filler_ai -p2 linux_robots/bender
./linux_game_engine -f maps/map01 -p1 solution/filler_ai -p2 linux_robots/h2_d2
./linux_game_engine -f maps/map01 -p1 solution/filler_ai -p2 linux_robots/wall_e

# For M1 Macs, use m1_game_engine and m1_robots
./m1_game_engine -f maps/map01 -p1 solution/filler_ai -p2 m1_robots/bender
```

## Available Opponent Bots (from docker_image/)

- **bender**: Medium difficulty bot
- **h2_d2**: Easy-medium difficulty bot  
- **wall_e**: Easy difficulty bot
- **terminator**: Very strong bot (optional to beat)

## Game Rules

1. Players start at opposite corners of the board (@ for Player 1, $ for Player 2)
2. Each turn, players receive a random Tetris-like piece
3. Pieces must be placed with **exactly one cell** overlapping existing territory

The AI implementation includes:

- **Expert Level**: Minimax with alpha-beta pruning (depth 6)
- **Hard Level**: Minimax with alpha-beta pruning (depth 4)
- **Medium Level**: Greedy strategy with heuristics
- **Easy Level**: Random move selection

### AI Evaluation Factors

- Territory control and expansion
- Strategic position control (center bias)
- Piece connectivity and efficiency
- Opponent blocking strategies
- Distance-based positioning

## Maps

The project includes three official maps:

- **map00**: 20x15 grid
- **map01**: 40x24 grid  
- **map02**: 100x100 grid

## Testing

Comprehensive test suite covering:

- Core game logic and rules validation
- AI strategy effectiveness
- Piece placement mechanics
- Edge cases and error handling
- Integration tests for complete game flows

```bash
cargo test --lib          # Unit tests
cargo test --test integration_tests  # Integration tests
```

## Performance Requirements

The AI is designed to meet audit requirements:

- **Win Rate**: 4/5 games against bender, h2_d2, and wall_e
- **Response Time**: Under 10 seconds per move
- **Memory Efficient**: Optimized for large boards (100x100)
- **Protocol Compliant**: Follows exact game engine communication format

## Bonus Features

- **Visualizer**: Real-time game visualization with animations
- **Human Player Mode**: Interactive gameplay with help system
- **Game Replay**: Record and playback game sessions
- **Multiple Game Modes**: Human vs AI, AI vs AI, Human vs Human
- **Terminator Challenge**: Advanced AI capable of competing against terminator bot

## Architecture

The codebase follows clean architecture principles:

- **Modular Design**: Separate concerns for game logic, AI, and visualization
- **Testable**: Comprehensive unit and integration test coverage
- **Extensible**: Easy to add new AI strategies or game modes
- **Performance Optimized**: Efficient algorithms for large-scale games

## Usage Examples

```bash
# Quick test against bender
./linux_game_engine -f maps/map01 -p1 solution/filler_ai -p2 linux_robots/bender

# Quiet mode for automated testing
./linux_game_engine -f maps/map00 -p1 solution/filler_ai -p2 linux_robots/wall_e -q

# With custom seed for reproducible results
./linux_game_engine -f maps/map02 -p1 solution/filler_ai -p2 linux_robots/h2_d2 -s 12345

# Throttled mode for visualization
./linux_game_engine -f maps/map01 -p1 solution/filler_ai -p2 linux_robots/bender -r
```

This implementation provides a complete, audit-ready Filler game that meets all functional and bonus requirements.

## Dependencies

- `rand = "0.8"` - Random number generation for pieces and AI
