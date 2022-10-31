# rustymines
A mine sweeping game written in Rust with optional dud mines.

## Parameters
You can specify the field size, amount of mines and amount of duds:

* `--width` The width of the field
* `--height` The height of the field
* `--mines` Amount of mines on the field
* `--duds` Amount of duds among the mines

## Playing
You play the game by either flagging of visiting fields.
The field is over, if you stepped onto a mine, which is not a dud, in which case you lose,
or you uncover all fields that do not contain a mine, in which case you win. 

### Visiting fields
You can visit fields under which you deem safe by specifying the x and y coordinate.
E.g.: `2 3` to visit the field at coordinate x=2 and y=3.

### Flagging fields
You can flag fields under which you suspect a mine by prepending an exclamation
mark to the coordinate. E.g.: `!3 1` to flag the field at coordinate x=3 and y=1.

### Uncovering all remaining fields
You can uncover all non-flagged fields by providing two exclamation marks: `!!`

### Aborting the game
You can abort and quit the game at any time by typing: `exit`
