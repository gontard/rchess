require("expose-loader?$!jquery");
import Chessboard from "chessboardjs";
import State from "./state";

const worker = new Worker("./worker.js");
const state = new State();
const board = Chessboard("board", {
  position: "start",
  showErrors: "console",
  showNotation: false,
  draggable: true,
  onDragStart: onDragStart,
  onDrop: onDrop
});

// only allow white pieces to be dragged
function onDragStart() {
  if (state.isBlackTurn()) {
    return false;
  }
}

function onDrop(source, target, piece, newUserPosition, oldUserPosition) {
  if (source === target) return;
  state.setBlackTurn();
  sendMovePiece(
    Chessboard.objToFen(oldUserPosition),
    Chessboard.objToFen(newUserPosition),
    { source, target }
  );
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
      }
    }
  });
}

function sendComputeMove() {
  worker.postMessage({
    type: "COMPUTE_MOVE"
  });
}

worker.addEventListener("message", event => {
  const message = event.data;
  const { type, payload } = message;
  if ("MOVE_PIECE_RESPONSE" === type) {
    const { previousPosition, newPosition } = payload;
    console.log("New IA position: " + newPosition);
    board.position(newPosition);
    if (newPosition !== previousPosition) {
      sendComputeMove();
    } else {
      state.setWhiteTurn();
    }
  } else if ("COMPUTE_MOVE_RESPONSE" === type) {
    const { newPosition } = payload;
    console.log("New IA position: " + newPosition);
    board.position(newPosition);
    state.setWhiteTurn();
  }
});

state.addEventListener("change", () => {
  $(".ia-working").toggleClass("visible", state.isBlackTurn());
});
