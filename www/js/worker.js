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
      const response = JSON.parse(rchess.move_piece(source + "-" + target));
      console.log("MOVE_PIECE response: ", response);
      postMovePieceResponse(previousPosition, response);
    } else if ("COMPUTE_MOVE" === type) {
      const response = JSON.parse(rchess.compute_move());
      console.log("COMPUTE_MOVE response: ", response);
      postComputeMoveResponse(response);
    }
  });

  function postMovePieceResponse(previousPosition, response) {
    self.postMessage({
      type: "MOVE_PIECE_RESPONSE",
      payload: {
        previousPosition,
        response
      }
    });
  }

  function postComputeMoveResponse(response) {
    self.postMessage({
      type: "COMPUTE_MOVE_RESPONSE",
      payload: {
        response
      }
    });
  }
});
