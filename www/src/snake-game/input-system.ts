import { InputDirection } from "snake-wasm";
import * as Hammer from "hammerjs";

const INPUT_KEY_MAPPINGS = {
    directions: {
        [InputDirection.UP]: ["W", "w", "ArrowUp"],
        [InputDirection.LEFT]: ["A", "a", "ArrowLeft"],
        [InputDirection.DOWN]: ["S", "s", "ArrowDown"],
        [InputDirection.RIGHT]: ["D", "d", "ArrowRight"],
    },
    stop: [" ", "Spacebar"],
};

const HAMMER_DIRECTION_MAPPINGS: { [key: number]: InputDirection } = {
    [Hammer.DIRECTION_UP]: InputDirection.UP,
    [Hammer.DIRECTION_LEFT]: InputDirection.LEFT,
    [Hammer.DIRECTION_DOWN]: InputDirection.DOWN,
    [Hammer.DIRECTION_RIGHT]: InputDirection.RIGHT,
};

export class InputSystem {
    private hammer: HammerManager;
    direction: InputDirection;
    onStop: () => void;

    public constructor(onStop = () => { }) {
        this.onStop = onStop;
        this.hammer = new Hammer.Manager(window);
        this.hammer.add(new Hammer.Tap({ event: 'doubletap', taps: 2 }));
        this.hammer.add(new Hammer.Pan({ direction: Hammer.DIRECTION_ALL }));
        this.hammer.on('doubletap', this.onDoubleTap.bind(this));
        this.hammer.on('pan', this.onPan.bind(this));
        window.addEventListener("keydown", this.onKeyDown.bind(this));
        window.addEventListener("keyup", this.onKeyUp.bind(this));
    }

    private onKeyDown(event: KeyboardEvent) {
        const input = Object.keys(INPUT_KEY_MAPPINGS.directions).find((key) =>
            INPUT_KEY_MAPPINGS.directions[parseInt(key)].includes(event.key)
        );
        if (input != undefined) {
            this.direction = parseInt(input);
        }
    }

    private onKeyUp(event: KeyboardEvent) {
        this.direction = undefined;
        if (INPUT_KEY_MAPPINGS.stop.includes(event.key)) {
            this.onStop();
        }
    }

    private onPan(event: HammerInput) {
        this.direction = HAMMER_DIRECTION_MAPPINGS[event.direction];
    }

    private onDoubleTap(event: HammerInput) {
        this.onStop();
        this.direction = undefined;
    }

}
