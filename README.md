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
4. Pieces cannot overlap opponent territory
5. The player controlling the most area when no valid moves remain wins

## AI Strategy

The AI uses different strategies based on difficulty:
- **Easy**: Random valid move selection
- **Medium**: Greedy strategy (maximize immediate territory gain)
- **Hard/Expert**: Minimax algorithm with alpha-beta pruning

## File Structure

```
src/
├── bin/
│   ├── filler_engine.rs    # Main game engine
│   └── filler_ai.rs        # Standalone AI bot
├── game.rs                 # Core game logic
├── piece.rs                # Piece generation and management
├── ai.rs                   # AI strategies and algorithms
├── player.rs               # Player interfaces
├── visualizer.rs           # Game visualization
├── utils.rs                # Utility functions
└── lib.rs                  # Library exports and tests

docker_image/               # Pre-built bots and game engines (DO NOT MODIFY)
├── linux_robots/          # Linux opponent bots
├── m1_robots/             # M1 Mac opponent bots
├── maps/                  # Official test maps
├── linux_game_engine      # Linux game engine
└── m1_game_engine         # M1 Mac game engine
```

## Testing

Run the test suite:
```bash
cargo test
```

## Dependencies

- `rand = "0.8"` - Random number generation for pieces and AI
