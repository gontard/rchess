import("../../crate-wasm/pkg").then(wasm => {
  const { RChess } = wasm;
  const rchess = RChess.new();
  console.log("instantiate RChess in worker");
  self.addEventListener("message", event => {
    const message = event.data;
    const { type, payload } = message;
    if ("MOVE_PIECE" === type) {
      const {
        previousPosition,
        move: { source, target }
      } = payload;
      const newIAPosition = rchess.move_piece(source + "-" + target);
      const newPosition = newIAPosition.split(" ")[0];
      sendMovePieceResponse(previousPosition, newPosition);
    } else if ("COMPUTE_MOVE" === type) {
      const d = new Date().getTime();
      const newIAPosition = rchess.compute_move();
      const d2 = new Date().getTime();
      console.log(`duration: ${(d2 - d) / 1000}s`);
      console.log("New IA position: " + newIAPosition);
      sendComputeMoveResponse(newIAPosition);
    }
  });

  function sendMovePieceResponse(previousPosition, newPosition) {
    self.postMessage({
      type: "MOVE_PIECE_RESPONSE",
      payload: {
        previousPosition,
        newPosition
      }
    });
  }

  function sendComputeMoveResponse(newPosition) {
    self.postMessage({
      type: "COMPUTE_MOVE_RESPONSE",
      payload: {
        newPosition
      }
    });
  }
});
