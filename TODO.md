# TODO List - Bevy Tetris

## Completed Features

- ✅ Wall kick system (SRS - Super Rotation System)
- ✅ Line elimination system
- ✅ Hard drop system
- ✅ Block rotation (Q/E keys)
- ✅ Block movement (Arrow keys)
- ✅ 7-bag randomizer
- ✅ Score system (line clears, hard drop, soft drop)
- ✅ Score and lines cleared UI display
- ✅ Soft drop (Arrow Down)
- ✅ Next piece preview (6-piece lookahead)
- ✅ Game over detection
- ✅ Hold piece functionality (C key)
- ✅ Level system (speed increases every 10 lines)
- ✅ Ghost piece (translucent landing preview)
- ✅ Lock delay (0.5s with move/rotate reset, max 15)
- ✅ Pause / Resume (P key)

## Completed Improvements

- ✅ Split main.rs into focused modules (board, movement, rotation, drop, line_clear, ghost, hold, game_state)
- ✅ Unit tests (26 tests covering board, drop, rotation, tetromino, randomizer)
- ✅ Ghost piece change tracking (skip rebuild when position unchanged)
- ✅ Hold preview change tracking (skip rebuild via block discriminant)
- ✅ Rotation in-place transform updates (no entity despawn/respawn)
- ✅ Lock delay cancellation when rotation/move opens space below
- ✅ Code cleanup (removed debug logging, commented-out code, unused test_block module)
- ✅ Refactored board_check_block_position to take &board instead of &mut ResMut

---

## Future Enhancements (Optional)

- High score persistence
