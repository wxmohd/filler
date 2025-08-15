# Filler Visualizer (Rust)

A terminal-based visualizer for the Filler game engine.  
This program reads the game engine's output and displays the Anfield, current piece, scores, and turn information in a clean, text-based format.

---

## Features

- Replay mode: load a saved game log and step through each turn.
- Live mode: run the game engine directly from the visualizer and display the game in real time.
- Board display using the same symbols as the game engine:
  - `@` → Player 1 old territory
  - `a` → Player 1 new piece
  - `$` → Player 2 old territory
  - `s` → Player 2 new piece
  - `.` → empty space
- Score counter: automatically counts the total territory for each player.
- Current piece preview.
- Playback controls for replay mode:
  - Space: play/pause
  - Left/Right arrow keys: move backward/forward one turn
  - R: restart
  - Q: quit

---

## Requirements

- Rust (stable version recommended)
- Cargo
- Filler game engine with maps and robots

---

## Installation

Clone the repository inside the container or on your host:

```bash
git clone `https://learn.reboot01.com/git/wamohamed/filler.git`

