# rustymines-tui

Play Rusty Mines in an interactive terminal.

```sh
cargo run -p rustymines-tui
cargo run -p rustymines-tui -- --width 16 --height 10 --mines 24 --duds 3
```

The default board is 8 × 8 with 10 mines and no duds. Duds count toward the mine total.

| Key | Action |
| --- | --- |
| Arrow keys | Move the highlighted selection. The board scrolls to follow it. |
| Enter or Space | Reveal the selected cell. |
| F | Toggle a flag on the selected cell. |
| ! | Reveal every unflagged cell, including any live mines among them. |
| R | Start a fresh board with the same settings. |
| Q, Esc, or Ctrl+C | Quit. |

A covered cell is `#` and a flag is `F`. A revealed safe cell shows its adjacent mine count, or `.` when that count is zero. A revealed dud is `~`, a detonated mine is `X`, and an untouched mine revealed at the end is `*`. Reveal every safe cell to win.
