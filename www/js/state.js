class State extends EventTarget {
  constructor() {
    super();
    this.whiteTurn = true;
  }

  setWhiteTurn() {
    this.whiteTurn = true;
    this.dispatchChangeEvent();
  }

  isWhiteTurn() {
    return this.whiteTurn;
  }

  setBlackTurn() {
    this.whiteTurn = false;
    this.dispatchChangeEvent();
  }

  isBlackTurn() {
    return !this.whiteTurn;
  }

  dispatchChangeEvent() {
    this.dispatchEvent(new Event("change"));
  }
}

export default State;
