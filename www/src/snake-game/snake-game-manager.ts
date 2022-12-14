import { SnakeGame, Vector } from 'snake-wasm'

import CONFIG from './config'
import { RenderingSystem as RenderingSystem } from './rendering-system';
import { Scoreboard } from './components/scoreboard';
import { CanvasContainer } from './components/canvas-container';
import { InputSystem } from './input-system';

export class SnakeDto {
    body: Array<Vector>;
    speed: number;
    direction: Vector;
}

export class SnakeGameManager {
    root: HTMLElement;

    game: SnakeGame;
    lastUpdate: number;
    stopTime: boolean;

    renderingSystem: RenderingSystem;
    inputSystem: InputSystem;

    scoreboard: Scoreboard;
    canvasContainer: CanvasContainer;

    constructor() {
        this.restart();
        this.scoreboard = new Scoreboard();
        this.canvasContainer = new CanvasContainer();
        this.root = document.createElement('div');
        this.root.id = 'container';
        this.root.appendChild(this.scoreboard.htmlElement);
        this.root.appendChild(this.canvasContainer.container);
        this.renderingSystem = new RenderingSystem(
            this.scoreboard,
            this.canvasContainer,
            this.game.board.width,
            this.game.board.height,
            this.render.bind(this)
        );
        this.inputSystem = new InputSystem(this.onStop.bind(this));
        this.lastUpdate = undefined;
        this.stopTime = undefined;
    }

    getRoot() {
        return this.root;
    }

    getBestScore() {
        return parseInt(localStorage.bestScore) || 0
    }

    setBestScore(bestScore: number) {
        localStorage.setItem('bestScore', bestScore.toString())
    }

    restart() {
        this.game = new SnakeGame(
            CONFIG.WIDTH,
            CONFIG.HEIGHT,
            CONFIG.SNAKE_INITIAL_LENGTH,
            CONFIG.SNAKE_INITIAL_SPEED,
            CONFIG.SNAKE_INITIAL_DIRECTION
        );
        this.lastUpdate = undefined
        this.stopTime = undefined
    }

    render() {
        this.renderingSystem.render(this.game.board.food, this.game.export_snake() as SnakeDto, this.game.score, this.getBestScore());
    }

    onUpdate() {
        if (!this.stopTime) {
            const now: number = Date.now();
            if (this.lastUpdate) {
                this.game.set_input_direction(this.inputSystem.direction);
                this.game.run_for(now - this.lastUpdate);
                if (this.game.is_over) {
                    this.restart();
                    return;
                }
                if (this.game.score > this.getBestScore()) {
                    this.setBestScore(this.game.score);
                }
            }
            this.lastUpdate = now;
            this.render();
        }
    }

    onStop() {
        const now: number = Date.now();
        if (this.stopTime) {
            this.stopTime = false;
            this.lastUpdate += now;
        } else {
            this.stopTime = true;
            this.lastUpdate -= now;
        }
    }

    run() {
        setInterval(this.onUpdate.bind(this), 1000 / CONFIG.FPS);
    }

}
