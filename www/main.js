require("expose-loader?$!jquery");
import Chessboard from "chessboardjs"

const worker = new Worker("./worker.js");

const board = Chessboard('board', {
  position: 'start',
  showErrors: 'console',
  draggable: true,
  onDrop: onDrop
})

function onDrop(source, target, piece, newUserPosition, oldUserPosition, orientation) {
  if (source === target) return
  sendMovePiece(Chessboard.objToFen(oldUserPosition), Chessboard.objToFen(newUserPosition), { source, target })
}

function sendMovePiece(previousPosition, newPosition, { source, target }) {
  worker.postMessage({
    type: "MOVE_PIECE",
    payload: {
      previousPosition,
      newPosition,
      move: {
        source,
        target
      },
    }
  })
}

function sendComputeMove() {
  worker.postMessage({
    type: "COMPUTE_MOVE"
  })
}


worker.addEventListener("message", event => {
  const message = event.data;
  const {type, payload} = message;
  if ("MOVE_PIECE_RESPONSE" === type) {
    const { previousPosition, newPosition } = payload
    console.log('New IA position: ' + newPosition)
    board.position(newPosition)
    if (newPosition !== previousPosition) {
      sendComputeMove()
    }
  }
  else if ("COMPUTE_MOVE_RESPONSE" === type) {
    const { newPosition } = payload
    console.log('New IA position: ' + newPosition)
    board.position(newPosition)
  }
});
