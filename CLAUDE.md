# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview

A Tetris game implementation using Bevy 0.15.1 game engine with ECS (Entity Component System) architecture. The game follows Guideline Tetris rules with a 7-bag randomizer.

## Build and Run Commands

```bash
# Run the game in development mode
cargo run

# Run with optimized dev build (recommended for better performance)
cargo run --release

# Build only
cargo build

# Build for release
cargo build --release

# Run with dev tools enabled (includes UI debug overlay)
cargo run --features bevy_dev_tools
```

## Development Profile Notes

The project uses custom optimization profiles in Cargo.toml:
- Dev profile: opt-level 1 for the main crate, opt-level 3 for dependencies (faster iteration with reasonable performance)
- Release profile: codegen-units=1 and lto="thin" for better optimization
- WASM release profile: opt-level="s" for smaller binary size

## Game Controls

- **Arrow Left/Right**: Move block horizontally
- **Arrow Up**: Hard drop (fast drop)
- **Q**: Rotate block counter-clockwise
- **E**: Rotate block clockwise
- **Space** (with bevy_dev_tools): Toggle UI debug overlay

## Architecture Overview

### ECS Pattern

The game follows Bevy's Entity Component System architecture:

**Components:**
- `ActiveBlock`: Marks the currently falling tetromino entity
- `ActiveDot`: Marks individual dots of the active block
- `BoardDot`: Marks dots that have been placed on the board (stores board_x, board_y coordinates)
- `Block` (enum): Represents tetromino types (I, O, T, S, Z, J, L) with rotation states and dot positions
- `Rotation`: Marker component for blocks that can rotate

**Resources:**
- `GameData`: Core game state containing:
  - `board_matrix`: 10x20 array representing the game board (0=empty, 1=occupied)
  - `drop_timer`: Controls normal drop speed (1.0s)
  - `hard_drop_timer`: Controls hard drop speed (0.01s)
  - `keyboard_timer`: Keyboard input timing (0.1s)
- `Randomizer7Bag`: Implements 7-bag randomizer for block spawning

**States:**
- `DropType`: Enum with `Normal` and `Hard` variants, controls drop behavior

### Core Systems

Systems run in the `Update` schedule with specific run conditions:

1. **spawn_block_system**: Spawns new blocks when no active block exists, uses 7-bag randomizer
2. **block_movement_system**: Handles horizontal movement (runs only in Normal drop state)
3. **block_rotation_system**: Handles Q/E rotation keys (runs only in Normal drop state)
4. **block_drop_type_system**: Switches to Hard drop state when Up arrow pressed
5. **block_drop_system**: Moves block down based on timers, places block when it can't drop further
6. **eliminate_line_system**: Detects and clears completed lines, moves remaining blocks down (runs in chain after block_drop_system)

### Coordinate Systems

**World Coordinates:**
- Origin at center of window (800x600)
- Each block dot is 25x25 pixels
- Blocks spawn at x=-37.5, y varies by type (I blocks at y=225.0, others at y=200.0)

**Board Coordinates:**
- 10 columns (x: 0-9), 20 rows (y: 0-19)
- Origin (0,0) is top-left
- Conversion functions: `get_object_position_in_board()`, `get_dot_position_in_board()`

### Module Structure

- **main.rs**: Main app setup, core game systems (movement, rotation, drop, line elimination), coordinate conversion utilities
- **tetromino.rs**: Block enum with 7 tetromino types, each storing 4 rotation states as dot arrays, rotation logic
- **spawn_block_system.rs**: Block spawning logic with 7-bag randomizer implementation
- **common_component.rs**: Shared components (ActiveBlock, ActiveDot, BoardDot) and resources (GameData, DropType)
- **background.rs**: Background and grid rendering
- **test_block.rs**: Testing utilities (currently commented out in main.rs)

## Key Implementation Details

### 7-Bag Randomizer

Implements Guideline Tetris randomization: shuffles all 7 tetromino types, dispenses them one by one, then refills and reshuffles. This ensures no piece appears twice in a row and provides more predictable piece distribution than pure random.

### Block Rotation

Each tetromino stores 4 rotation states (Zero, One, Two, Three) as arrays of 4 dots with relative coordinates. Rotation recreates child entities with new dot positions. The `kick_check()` function exists but is not yet implemented (wall kicks not functional).

### Line Elimination

The `eliminate_line_system` runs in a chain after `block_drop_system`:
1. Scans board_matrix for completed lines (all 1s)
2. Despawns dot entities in completed lines
3. Recursively moves lines down using `eliminate_line_inner()`
4. Updates BoardDot components and transforms for remaining dots

### Block Placement

When a block can't drop further, `place_block_on_board()`:
1. Removes parent-child relationships
2. Converts ActiveDot components to BoardDot components with board coordinates
3. Updates board_matrix
4. Despawns the parent block entity

## UI

Uses bevy_egui to display a debug window showing the board_matrix state as a grid of filled (■) and empty (□) squares.

## Known Incomplete Features

- Wall kicks are not implemented (kick_check function is a stub)
- No game over detection
- No scoring system
- No level progression or speed increase
- No hold piece functionality
- No next piece preview
