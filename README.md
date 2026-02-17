# bevy-tetris

A Tetris game implementation using the Bevy game engine (v0.15.1) with ECS architecture and modern Tetris guidelines.

## Features

- ✅ **SRS Wall Kicks**: Full Super Rotation System implementation with proper kick tables for all piece types
- ✅ **7-Bag Randomizer**: Guideline-compliant piece randomization for fair gameplay
- ✅ **Line Clearing**: Automatic line detection and clearing with proper block movement
- ✅ **Hard Drop**: Fast drop functionality with visual feedback
- ✅ **Smooth Controls**: Responsive keyboard input for movement and rotation
- ✅ **Debug UI**: Real-time board state visualization using egui

## Requirements

- Rust 1.93.1 or later (for edition2024 support)
- Cargo 1.93.1 or later

## Installation

```bash
# Clone the repository
git clone <repository-url>
cd bevy-tetris

# Build the project
cargo build --release
```

## Running the Game

```bash
# Run in development mode (faster compilation)
cargo run

# Run in release mode (better performance)
cargo run --release

# Run with dev tools enabled (UI debug overlay)
cargo run --features bevy_dev_tools
```

## Controls

| Key | Action |
|-----|--------|
| ← → | Move block left/right |
| ↑ | Hard drop (fast drop) |
| Q | Rotate counter-clockwise |
| E | Rotate clockwise |
| Space | Toggle UI debug overlay (with bevy_dev_tools) |

## Game Rules

- **Board Size**: 10 columns × 20 rows
- **Piece Types**: I, O, T, S, Z, J, L (standard Tetris pieces)
- **Rotation System**: SRS (Super Rotation System) with wall kicks
- **Randomizer**: 7-bag system (all 7 pieces appear once before reshuffling)
- **Drop Speed**: 1 second per row (normal), 0.01 seconds per row (hard drop)

## Architecture

### ECS Components

- **ActiveBlock**: Marks the currently falling tetromino
- **ActiveDot**: Individual dots of the active block
- **BoardDot**: Placed dots with board coordinates
- **Block**: Tetromino type with rotation states
- **Rotation**: Marker for rotatable blocks

### Resources

- **GameData**: Core game state (board matrix, timers, score)
- **Randomizer7Bag**: 7-bag piece randomization
- **DropType**: State machine for normal/hard drop

### Key Systems

1. **spawn_block_system**: Spawns new blocks using 7-bag randomizer
2. **block_movement_system**: Handles horizontal movement
3. **block_rotation_system**: Handles rotation with SRS wall kicks
4. **block_drop_system**: Manages block falling and placement
5. **eliminate_line_system**: Detects and clears completed lines

## Project Structure

```
bevy-tetris/
├── src/
│   ├── main.rs                  # Main game loop and systems
│   ├── tetromino.rs             # Block definitions and rotation logic
│   ├── spawn_block_system.rs   # Block spawning and randomizer
│   ├── common_component.rs      # Shared components and resources
│   ├── background.rs            # Background rendering
│   └── test_block.rs            # Testing utilities
├── assets/                      # Game assets (if any)
├── Cargo.toml                   # Project dependencies
├── CLAUDE.md                    # AI assistant guidance
├── TODO.md                      # Development roadmap
└── README.md                    # This file
```

## Development

### Building

The project uses custom optimization profiles for better development experience:

- **Dev profile**: opt-level 1 for main crate, opt-level 3 for dependencies
- **Release profile**: Full optimization with LTO
- **WASM release**: Size-optimized for web deployment

### Testing

```bash
# Run tests
cargo test

# Run with logging
RUST_LOG=debug cargo run
```

## Planned Features

See [TODO.md](TODO.md) for the complete development roadmap. Upcoming features include:

- Score system with line clear bonuses
- Level progression with increasing difficulty
- Next piece preview
- Hold piece functionality
- Game over detection
- High score persistence

## Technical Details

- **Engine**: Bevy 0.15.1
- **UI**: bevy_egui 0.32.0
- **Randomization**: rand 0.8.5
- **Graphics Backend**: Metal (macOS), Vulkan/DirectX (other platforms)
- **Coordinate System**: 25×25 pixel blocks, centered origin

## Known Issues

- Wall kick implementation is complete but may need fine-tuning for edge cases
- No game over detection yet
- Score system not yet implemented

## Contributing

Contributions are welcome! Please feel free to submit issues or pull requests.

## License

[Add your license here]

## Credits

Built with [Bevy](https://bevyengine.org/) - A refreshingly simple data-driven game engine built in Rust.
