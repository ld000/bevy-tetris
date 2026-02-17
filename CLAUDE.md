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

# Run tests
cargo test

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
- **Arrow Down**: Soft drop (faster drop, 1 point per cell)
- **Q**: Rotate block counter-clockwise
- **E**: Rotate block clockwise
- **C**: Hold piece (swap current piece with hold)
- **P**: Pause / Resume game
- **Enter**: Restart (on game over screen)
- **Space** (with bevy_dev_tools): Toggle UI debug overlay

## Architecture Overview

### ECS Pattern

The game follows Bevy's Entity Component System architecture:

**Components:**
- `ActiveBlock`: Marks the currently falling tetromino entity
- `ActiveDot`: Marks individual dots of the active block
- `BoardDot`: Marks dots that have been placed on the board (stores board_x, board_y coordinates)
- `GhostDot`: Marks translucent ghost piece dots showing landing preview
- `Block` (enum): Represents tetromino types (I, O, T, S, Z, J, L) with rotation states and dot positions
- `Rotation`: Marker component for blocks that can rotate
- `PauseOverlay`: Marks the pause screen UI overlay

**Resources:**
- `GameData`: Core game state containing:
  - `board_matrix`: 10x20 array representing the game board (0=empty, 1=occupied)
  - `drop_timer`: Controls normal drop speed (1.0s)
  - `hard_drop_timer`: Controls hard drop speed (0.01s)
  - `soft_drop_timer`: Controls soft drop speed (0.05s)
  - `score`, `lines_cleared`: Scoring state
  - `held_block`: Currently held piece (Option<Block>)
  - `hold_used`: Whether hold has been used this turn
  - `lock_delay_timer`: 0.5s timer for lock delay (once mode)
  - `lock_delay_active`: Whether lock delay is currently counting down
  - `lock_move_count`: Number of move/rotate resets during lock delay (capped at 15)
- `Randomizer7Bag`: Implements 7-bag randomizer for block spawning
- `GhostTracker`: Tracks ghost piece position/rotation to avoid per-frame entity churn
- `HoldTracker`: Tracks hold preview state by block discriminant to skip unnecessary rebuilds

**States:**
- `DropType`: Enum with `Normal`, `Hard`, and `Soft` variants, controls drop behavior
- `GameState`: Enum with `Playing`, `Paused`, and `GameOver` variants

### Core Systems

Systems run in the `Update` schedule with specific run conditions:

1. **spawn_block_system**: Spawns new blocks when no active block exists, uses 7-bag randomizer. Detects game over if spawn position is blocked.
2. **block_movement_system**: Handles horizontal movement (runs only in Normal/Soft drop state)
3. **block_rotation_system**: Handles Q/E rotation keys with SRS wall kicks (runs only in Normal/Soft drop state)
4. **block_drop_type_system**: Switches drop state (Up=Hard, Down=Soft)
5. **block_drop_system**: Moves block down based on timers, initiates lock delay when piece can't drop, places block when lock delay expires. Cancels lock delay if a rotation/move opens space below. Hard drop bypasses lock delay.
6. **eliminate_line_system**: Detects and clears completed lines, awards score, moves remaining blocks down (runs in chain after block_drop_system)
7. **hold_block_system**: Handles C key to swap current piece with held piece
8. **update_hold_preview_system**: Renders held piece preview in left panel (skips rebuild when hold state unchanged)
9. **update_preview_system**: Renders next 6 pieces in right panel
10. **update_ghost_piece_system**: Renders translucent ghost dots at the landing position (skips rebuild when position/rotation unchanged)
11. **game_over_display_system**: Shows game over overlay with score
12. **restart_system**: Handles Enter key to restart from game over
13. **pause_system**: Toggles between Playing and Paused states on P key press
14. **pause_display_system**: Spawns pause overlay UI (OnEnter Paused)
15. **unpause_cleanup_system**: Despawns pause overlay (OnExit Paused)

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

- **main.rs**: App setup, mod declarations, system registration, egui debug UI
- **board.rs**: `BoardDot` component, coordinate conversion (`get_object_position_in_board`, `get_dot_position_in_board`), `board_check_block_position`, `place_dot_on_board`
- **movement.rs**: `block_movement_system` — horizontal movement with lock delay reset
- **rotation.rs**: `block_rotation_system`, `get_kick_offsets` — SRS wall kicks, in-place child transform updates
- **drop.rs**: `gravity_seconds`, `block_drop_type_system`, `block_drop_system`, `place_block_on_board` — drop logic with lock delay
- **line_clear.rs**: `eliminate_line_system`, `eliminate_line_inner` — line detection, scoring, recursive line shifting
- **ghost.rs**: `update_ghost_piece_system`, `GhostTracker` — ghost piece with change-detection optimization
- **hold.rs**: `hold_block_system`, `update_hold_preview_system`, `HoldTracker` — hold piece swap and preview rendering
- **game_state.rs**: `update_score_display`, `pause_system`, `pause_display_system`, `unpause_cleanup_system`, `game_over_display_system`, `restart_system`
- **tetromino.rs**: `Block` enum with 7 tetromino types, each storing 4 rotation states as dot arrays, `Rotation` component and logic
- **spawn_block_system.rs**: Block spawning logic with `Randomizer7Bag` implementation, next piece preview
- **common_component.rs**: Shared components (`ActiveBlock`, `ActiveDot`, etc.) and resources (`GameData`, `DropType`, `GameState`)
- **background.rs**: Background, grid rendering, score display layout

## Key Implementation Details

### 7-Bag Randomizer

Implements Guideline Tetris randomization: shuffles all 7 tetromino types, dispenses them one by one, then refills and reshuffles. This ensures no piece appears twice in a row and provides more predictable piece distribution than pure random.

### Block Rotation

Each tetromino stores 4 rotation states (Zero, One, Two, Three) as arrays of 4 dots with relative coordinates. Rotation updates child entity transforms in place (no despawn/respawn). Full SRS (Super Rotation System) wall kicks are implemented via `get_kick_offsets()` in `rotation.rs`.

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
5. Resets lock delay state and hold availability

### Lock Delay

When a piece lands (non-hard-drop), a 0.5s lock delay timer starts instead of instant placement. The timer is ticked every frame (independent of the drop timer). Moving or rotating the piece resets the timer, up to 15 resets to prevent infinite stalling. If a rotation/move opens space below the piece, lock delay is cancelled and normal gravity resumes. Hard drop bypasses lock delay entirely.

### Ghost Piece

The `update_ghost_piece_system` simulates dropping the active piece from its current position until it can't go further, then spawns translucent (20% opacity) sprites at the landing position. A `GhostTracker` resource caches the last rendered position and rotation state — ghost dots are only rebuilt when the piece moves or rotates.

## Testing

Unit tests are in inline `#[cfg(test)] mod tests` blocks within each module. Run with `cargo test`. Coverage includes:
- **board.rs**: Coordinate conversion, collision detection, dot placement
- **drop.rs**: Gravity curve values, monotonic decrease, floor at 0.05s
- **rotation.rs**: SRS kick offset counts, identity-first property
- **tetromino.rs**: Rotation cycling, state reset, O-piece invariance
- **spawn_block_system.rs**: 7-bag completeness, peek/pop behavior, auto-refill

## UI

Uses bevy_egui to display a debug window showing the board_matrix state as a grid of filled (■) and empty (□) squares.

