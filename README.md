# Scrabby

Scrabby is a CLI application that will find the best move on a Super Scrabble board given the board and your rack.

---

> **NOTE: The Super Scrabble board is different from normal Scrabble, so the best move on a Super Scrabble board may be different than on a normal Scrabble board. Additionally, blank pieces are not supported yet, so substitute it for any letter you like.**

## Building

```bash
git clone https://www.github.com/Dark42ed/scrabby.git
cd scrabby
cargo build --release
```

## Usage

1. **Other person's turn**
    This is where you update the board. When somebody moves, you enter in the word, location, and direction. The board will then be updated and then printed out.
2. **Your turn**
    This is where the magic happens. You enter in your rack, and the computer will find the best word for you to make, highlighted in red. If you want a different word (or the people you are playing with thinks it's not a real word), then simply hit enter and it will show the next best word. When you are finished, just type "done". *(The board will not update, so you must update it with the word you play)*.