# TODO List - Bevy Tetris

## Score System Implementation

### Task 1: Add score field to GameData
**Status**: Pending
**File**: `src/common_component.rs`
**Description**: Add score and lines_cleared fields to the GameData struct to track player score and number of lines cleared.

**Changes needed**:
- Add `pub score: u32` field to GameData
- Add `pub lines_cleared: u32` field to GameData
- Initialize both fields to 0 in the Default impl

---

### Task 2: Implement scoring logic for line clears
**Status**: Pending
**File**: `src/main.rs` (eliminate_line_system function)
**Description**: Update the eliminate_line_system function to calculate and award points based on number of lines cleared.

**Scoring rules**:
- 1 line cleared = 100 points
- 2 lines cleared = 300 points
- 3 lines cleared = 500 points
- 4 lines cleared = 800 points

**Changes needed**:
- Count the number of lines cleared in eliminate_line_system
- Calculate score based on the number of lines
- Update game_data.score and game_data.lines_cleared

---

### Task 3: Add hard drop scoring
**Status**: Pending
**File**: `src/main.rs` (block_drop_system function)
**Description**: Add scoring for hard drops - award 2 points per cell dropped during hard drop.

**Changes needed**:
- Track the distance dropped during hard drop
- Award 2 points per cell dropped
- Update game_data.score

---

### Task 4: Update UI to display score and lines cleared
**Status**: Pending
**File**: `src/main.rs` (ui_example_system function)
**Description**: Update the egui window to display the current score and lines cleared count.

**Changes needed**:
- Add score display to the UI window
- Add lines cleared display to the UI window
- Format the display nicely (e.g., "Score: 1200" and "Lines: 12")

---

## Completed Features

- ✅ Wall kick system (SRS - Super Rotation System)
- ✅ Line elimination system
- ✅ Hard drop system
- ✅ Block rotation (Q/E keys)
- ✅ Block movement (Arrow keys)
- ✅ 7-bag randomizer

---

## Future Enhancements (Optional)

- Level system (increase drop speed as score increases)
- Soft drop scoring (1 point per cell)
- Next piece preview
- Hold piece functionality
- Game over detection
- High score persistence
