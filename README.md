# Simple chess AI

A simple chess AI based on this tutorial: [A step-by-step guide to building a simple chess AI](https://www.freecodecamp.org/news/simple-chess-ai-step-by-step-1d55a9266977/) (https://github.com/lhartikk/simple-chess-ai).
It served as a support to continue to learn rust and to (re)discover a bit some basic game algorithm.

## Features

This set of features are implemented in the algorithm:
- [minimax algorithm](https://www.chessprogramming.org/Minimax)
- [alpha-beta pruning](https://www.chessprogramming.org/Alpha-Beta)
- [transposition table](https://www.chessprogramming.org/Transposition_Table)

## Architecture
The design is simple everything runs in the browser.

### AI implementation
The AI algorithm runs in a dedicated web worker thread since this is a CPU intensive task. Thus the UI rendering is not blocked.

The algorithm is a [web assembly](https://webassembly.org/) module compiled from rust using [rustwasm](https://rustwasm.github.io/).

### UI

The UI is very basic but it is good enough.
[chessboardjs](https://chessboardjs.com/) is used to display the chessboard. 

## Known limitations

Some chess moves are not possible to the user (resignation, promotion).
The algorithm  is not very good at the end-game it should have a more specific behavior.
