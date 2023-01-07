import { Food, Vector as Vector } from "snake-wasm";
import { CanvasContainer, CanvasResizeEvent } from "./components/canvas-container";
import { SnakeDto } from "./snake-game-manager";

export class RenderingSystem {
    canvasContainer: CanvasContainer;
    gameWidth: number;
    gameHeight: number;
    unitOnScreen: number;
    projectScalar: (scalar: number, unitOnScreen: number) => number;
    projectVector: (vector: Vector, unitOnScreen: number) => Vector;

    constructor(canvasContainer: CanvasContainer, gameWidth: number, gameHeight: number, onResize = () => { }) {
        this.canvasContainer = canvasContainer;
        this.gameWidth = gameWidth;
        this.gameHeight = gameHeight;
        this.projectScalar = (distance, unitOnScreen) => distance * unitOnScreen;
        this.projectVector = (distance, unitOnScreen) => new Vector(distance.x * unitOnScreen, distance.y * unitOnScreen);
        this.canvasContainer.onContainerResize(() => {
            this.setUpCanvas();
            onResize();
        });
        this.setUpCanvas();
    }

    setUpCanvas() {
        this.canvasContainer.resetCanvas();
        const container = this.canvasContainer.container;
        const canvas = this.canvasContainer.canvas;
        const context = this.canvasContainer.context;
        const { width, height } = container.getBoundingClientRect();
        this.unitOnScreen = Math.min(width / this.gameWidth, height / this.gameHeight);
        canvas.setAttribute('width', this.projectScalar(this.gameWidth, this.unitOnScreen).toString());
        canvas.setAttribute('height', this.projectScalar(this.gameHeight, this.unitOnScreen).toString());
        context.clearRect(0, 0, this.canvasContainer.canvas.width, this.canvasContainer.canvas.height);
        document.dispatchEvent(new CanvasResizeEvent(canvas.width, canvas.height));
    }

    render(food: Food, snake: SnakeDto, score: number, bestScore: number) {
        const context = this.canvasContainer.context;
        const canvas = this.canvasContainer.canvas;
        context.clearRect(0, 0, canvas.width, canvas.height);
        this.renderFood(food);
        this.renderSnake(snake);
    }

    private renderFood(food: Food) {
        const context = this.canvasContainer.context;
        const projectedFoodPosition: Vector = this.projectVector(food.position, this.unitOnScreen);
        context.beginPath();
        context.arc(projectedFoodPosition.x, projectedFoodPosition.y, this.unitOnScreen / 2.5, 0, 2 * Math.PI);
        context.lineWidth = 5;
        context.strokeStyle = '#3b4a2a';
        context.stroke();
    }

    private renderSnake(snake: SnakeDto) {
        const context = this.canvasContainer.context;
        context.lineWidth = this.unitOnScreen;
        context.strokeStyle = '#3b4a2a'
        context.beginPath();
        snake.body.map(v => this.projectVector(v, this.unitOnScreen))
            .forEach(({ x, y }) => context.lineTo(x, y));
        context.stroke();
    }
}
