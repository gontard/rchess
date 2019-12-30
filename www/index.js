require("expose-loader?$!jquery");
import {RChess} from "hello-wasm-pack";
import Chessboard from "chessboardjs"

const rchess = RChess.new()
const board = Chessboard('board', {
    position: 'start',
    showErrors: 'console',
    draggable: true,
    onDrop: onDrop
})

function onDrop(source, target, piece, newUserPosition, oldUserPosition, orientation) {
    if (source === target) return
    const newIAPosition = rchess.move_piece(source + "-" + target)
    const newPosition = newIAPosition.split(' ')[0];
    console.log(Chessboard.objToFen(oldUserPosition))
    console.log(newPosition)
    setTimeout(() => board.position(newPosition), 0)
    if (newPosition !== Chessboard.objToFen(oldUserPosition)) {
        setTimeout(() => {
            var d = new Date().getTime();
            const newIAPosition = rchess.compute_move()
            var d2 = new Date().getTime();
            board.position(newIAPosition)
            console.log('New ia position: ' + newIAPosition)
            console.log(`duration: ${(d2 - d) / 1000}s`)
            board.position(newIAPosition)
        }, 500)

    } else {
    }
}

// const interval = setInterval(step, 200)
// board.addListener
//
// function step() {
//     console.log(move)
//     if (!move) {
//         clearInterval(interval)
//     }
//     board.position(move)
// }

