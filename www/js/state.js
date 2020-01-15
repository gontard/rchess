class State extends EventTarget {
    constructor() {
        super();
        this.color = 'white';
        this.position = null;
        this.handleMessage = this.handleMessage.bind(this)
        this.worker = new Worker("./worker.js");
        this.worker.addEventListener("message", this.handleMessage);
    }

    setColor(newColor) {
        let oldColor = this.color
        this.color = newColor;
        this.dispatchPropertyChangeEvent('color', oldColor, newColor);
    }

    isWhiteTurn() {
        return this.color === 'white';
    }

    isBlackTurn() {
        return this.color === 'black';
    }

    setPosition(newPosition) {
        let oldPosition = this.position
        this.position = newPosition
        console.log("new position: " + newPosition);
        this.dispatchPropertyChangeEvent('position', oldPosition, newPosition)
    }

    getPosition() {
        return this.position
    }

    movePiece(previousPosition, newPosition, {source, target}) {
        this.setColor('black');
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
        const {type, payload} = message;
        if ("MOVE_PIECE_RESPONSE" === type) {
            const {previousPosition, response: {position}} = payload;
            console.log(payload)
            const newPosition = position.split(" ")[0];
            this.setPosition(newPosition);
            if (newPosition !== previousPosition) {
                this.computeMove();
            } else {
                this.setColor("white");
            }
        } else if ("COMPUTE_MOVE_RESPONSE" === type) {
            const {response: {position}} = payload;
            console.log("New IA position: " + position);
            this.setPosition(position);
            this.setColor("white");
        }
    }

    dispatchPropertyChangeEvent(propertyName, oldValue, newValue) {
        this.dispatchEvent(new CustomEvent(propertyName + "Changed", {oldValue, newValue}));
    }
}

export default State;
