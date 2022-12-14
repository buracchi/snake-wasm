import { InputDirection } from "snake-wasm";

const DIRECTION_INPUT_KEYS = {
    [InputDirection.UP]: ["W", "w", "ArrowUp"],
    [InputDirection.LEFT]: ["A", "a", "ArrowLeft"],
    [InputDirection.DOWN]: ["S", "s", "ArrowDown"],
    [InputDirection.RIGHT]: ["D", "d", "ArrowRight"],
}

const STOP_KEY: Array<string> = [" ", "Spacebar"];

export class InputSystem {
    direction: InputDirection;

    constructor(onStop = () => { }) {
        window.addEventListener('keydown', (e) => {
            let input = Object.keys(DIRECTION_INPUT_KEYS).find(key => DIRECTION_INPUT_KEYS[parseInt(key)].includes(e.key));
            if (input != undefined) {
                this.direction = parseInt(input);
            }
        });
        window.addEventListener('keyup', (e) => {
            this.direction = undefined;
            if (STOP_KEY.includes(e.key)) {
                onStop();
            }
        });
    }
}
