require("expose-loader?$!jquery");
import Chessboard from "chessboardjs";
import State from "./state";

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
  state.movePiece(
      Chessboard.objToFen(oldUserPosition),
      Chessboard.objToFen(newUserPosition),
      { source, target }
  )
}

state.addEventListener("colorChanged", () => {
  $(".ia-working").toggleClass("visible", state.isBlackTurn());
});

state.addEventListener("positionChanged", () => {
  board.position(state.getPosition());
});
