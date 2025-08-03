# rustymines-cli

A mine sweeping game for the console with optional dud mines.

## Parameters

You can specify the field size, amount of mines and amount of duds:

* `--width` The width of the field
* `--height` The height of the field
* `--mines` Amount of mines on the field
* `--duds` Amount of duds among the mines

## Playing

You play the game by either flagging or visiting fields.
The game is over, if you stepped onto a mine, which is not a dud, in which case you lose,
or you uncover all fields that do not contain a mine, in which case you win.

### Visiting fields

You can visit fields, which you deem safe, by specifying the x and y coordinate.
E.g.: `2 3` to visit the field at coordinate x=2 and y=3.

### Flagging fields

You can flag fields under which you suspect a mine by prepending an exclamation
mark to the coordinate. E.g.: `!3 1` to flag the field at coordinate x=3 and y=1.

You can remove a flag from a field by repeating the command, i.e. this command toggles a field's flag.

### Uncovering all remaining fields

You can uncover all non-flagged fields by providing two exclamation marks: `!!`

### Aborting the game

You can abort and quit the game at any time by typing: `exit`

### Symbols

- `■`: An unvisited field.
- `*`: A mine that has not been triggered.
- ` ` (a space): A cleared field with no surrounding mines.
- `1..8` (a decimal digit): A cleared field with the respective amount of surrounding mines.
- `☠`: A mine that has been stepped on and exploded. Better luck next time!
- `~`: A mine that has been stepped on, but turned out to be a dud. Lucky you!
- `⚐`: A flagged field with a potential mine underneath it. Flagged fields cannot be accidentally stepped on.
