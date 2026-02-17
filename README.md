# bevy-tetris

A Tetris game built with the Bevy 0.15.1 game engine, following Guideline Tetris rules.

## Features

- **SRS Wall Kicks**: Full Super Rotation System with kick tables for all piece types
- **7-Bag Randomizer**: Guideline-compliant piece randomization
- **Scoring**: Line clears (100/300/500/800), hard drop (2pts/cell), soft drop (1pt/cell)
- **Line Clearing**: Automatic detection and clearing with gravity
- **Hard Drop / Soft Drop**: Fast drop and accelerated drop
- **Next Piece Preview**: Shows the next 6 upcoming pieces
- **Game Over & Restart**: Detection when blocks can't spawn, press Enter to restart
- **Score Display**: Real-time score and lines cleared in the side panel
- **Debug UI**: Board state visualization using egui

## Requirements

- Rust (edition 2021)

## Getting Started

```bash
git clone <repository-url>
cd bevy-tetris
cargo run --release
```

## Controls

| Key | Action |
|-----|--------|
| ← → | Move block left/right |
| ↑ | Hard drop |
| ↓ | Soft drop |
| Q | Rotate counter-clockwise |
| E | Rotate clockwise |
| Enter | Restart (on game over) |
| Space | Toggle debug overlay (with bevy_dev_tools) |

## Game Rules

- **Board**: 10 × 20
- **Pieces**: I, O, T, S, Z, J, L
- **Rotation**: SRS with wall kicks
- **Randomizer**: 7-bag (all 7 pieces before reshuffling)
- **Drop Speed**: 1s/row normal, 0.05s/row soft, 0.01s/row hard

## Project Structure

```
src/
├── main.rs                # App setup, core systems (movement, rotation, drop, scoring, game over)
├── tetromino.rs           # Block types, rotation states, SRS kick tables
├── spawn_block_system.rs  # Block spawning, 7-bag randomizer, next piece preview
├── common_component.rs    # Components (ActiveBlock, ActiveDot, BoardDot) and resources (GameData)
├── background.rs          # Background, grid, score panel rendering
└── test_block.rs          # Testing utilities
```

## Technical Details

- **Engine**: Bevy 0.15.1
- **UI**: bevy_egui 0.32.0
- **Randomization**: rand 0.8.5
- **Coordinate System**: 25×25 pixel blocks, centered origin (800×600 window)

## Planned Features

- Level progression with increasing speed
- Hold piece functionality
- High score persistence

## License

[Add your license here]
