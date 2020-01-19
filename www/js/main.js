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
function onDragStart(source, piece) {
  const isBlackPiece = piece[0] === "b";
  if (isBlackPiece || state.isBlackTurn() || state.isFinished()) {
    return false;
  }
}

function onDrop(source, target, piece, newUserPosition, oldUserPosition) {
  if (source === target) return;
  state.movePiece(
    Chessboard.objToFen(oldUserPosition),
    Chessboard.objToFen(newUserPosition),
    { source, target }
  );
}

state.addEventListener("colorChanged", () => {
  $(".ia-working").toggleClass("visible", state.isBlackTurn());
});

state.addEventListener("positionChanged", () => {
  board.position(state.getPosition());
});

state.addEventListener("resultChanged", () => {
  if (state.isFinished()) {
    const messages = {
      WhiteCheckmates: "The white checkmates",
      WhiteResigns: "The white resigns",
      BlackCheckmates: "The black checkmates",
      BlackResigns: "The black resigns",
      Stalemate: "Stalemate (Draw)",
      DrawAccepted: "Draw accepted",
      DrawDeclared: "Draw declared"
    };
    let resultElement = $(".end");
    $(".endgame").addClass("visible");
    $(".endgame .text").text(messages[state.getResult()]);
  }
});
