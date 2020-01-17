class State extends EventTarget {
  constructor() {
    super();
    this.color = "white";
    this.position = null;
    this.result = null;
    this.handleMessage = this.handleMessage.bind(this);
    this.worker = new Worker("./worker.js");
    this.worker.addEventListener("message", this.handleMessage);
  }

  setColor(color) {
    this.color = color;
    this.dispatchPropertyChangeEvent("color", color);
  }

  isWhiteTurn() {
    return this.color === "white";
  }

  isBlackTurn() {
    return this.color === "black";
  }

  setPosition(position) {
    this.position = position;
    this.dispatchPropertyChangeEvent("position");
  }

  getPosition() {
    return this.position;
  }

  setResult(result) {
    this.result = result;
    this.dispatchPropertyChangeEvent("result");
  }

  getResult() {
    return this.result;
  }

  isFinished() {
    return !!this.result;
  }

  movePiece(previousPosition, newPosition, { source, target }) {
    this.setColor("black");
    this.worker.postMessage({
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

  computeMove() {
    this.worker.postMessage({
      type: "COMPUTE_MOVE"
    });
  }

  handleMessage(event) {
    const message = event.data;
    const { type, payload } = message;
    if ("MOVE_PIECE_RESPONSE" === type) {
      const {
        previousPosition,
        response: { position, result }
      } = payload;
      const newPosition = position.split(" ")[0];
      this.setPosition(newPosition);
      if (result) {
        this.setResult(result);
      } else if (newPosition !== previousPosition) {
        this.computeMove();
      } else {
        this.setColor("white");
      }
    } else if ("COMPUTE_MOVE_RESPONSE" === type) {
      const {
        response: { position, result }
      } = payload;
      this.setPosition(position);
      this.setColor("white");
      if (result) {
        this.setResult(result);
      }
    }
  }

  dispatchPropertyChangeEvent(propertyName) {
    this.dispatchEvent(new Event(propertyName + "Changed"));
  }
}

export default State;
