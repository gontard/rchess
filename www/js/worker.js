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
      const response = rchess.move_piece(source + "-" + target);
      console.log(response)
      sendMovePieceResponse(previousPosition, JSON.parse(response));
    } else if ("COMPUTE_MOVE" === type) {
      const d = new Date().getTime();
      const response = rchess.compute_move();
      const d2 = new Date().getTime();
      console.log(`duration: ${(d2 - d) / 1000}s`);
      console.log("New IA response: " + response);
      sendComputeMoveResponse(JSON.parse(response));
    }
  });

  function sendMovePieceResponse(previousPosition, response) {
    self.postMessage({
      type: "MOVE_PIECE_RESPONSE",
      payload: {
        previousPosition,
        response
      }
    });
  }

  function sendComputeMoveResponse(response) {
    self.postMessage({
      type: "COMPUTE_MOVE_RESPONSE",
      payload: {
        response
      }
    });
  }
});
