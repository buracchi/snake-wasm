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
        this.inputSystem = new InputSystem(this.onStop.bind(this));
        this.scoreboard = new Scoreboard();
        this.canvasContainer = new CanvasContainer();
        this.root = document.createElement('div');
        this.root.id = 'container';
        this.root.appendChild(this.scoreboard.htmlElement);
        this.root.appendChild(this.canvasContainer.container);
        this.restart();
        this.renderingSystem = new RenderingSystem(
            this.canvasContainer,
            this.game.board.width,
            this.game.board.height,
            this.render.bind(this)
        );
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
        this.inputSystem.direction = undefined;
        this.lastUpdate = undefined
        this.stopTime = undefined
    }

    render() {
        this.renderingSystem.render(this.game.board.food, this.game.export_snake() as SnakeDto, this.game.score, this.getBestScore());
    }

    updateScore() {
        if (this.game.score > this.getBestScore()) {
            this.setBestScore(this.game.score);
        }
        this.scoreboard.setCurrentScore(this.game.score);
        this.scoreboard.setBestScore(this.getBestScore());
    }

    onUpdate() {
        if (this.game.is_over) {
            this.restart();
        }
        else if (!this.stopTime) {
            const now: number = Date.now();
            if (this.lastUpdate) {
                this.game.set_input_direction(this.inputSystem.direction);
                this.game.run_for(now - this.lastUpdate);
                this.updateScore();
            }
            this.lastUpdate = now;
            this.render();
        }
        requestAnimationFrame(this.onUpdate.bind(this));
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

    startGame() {
        this.onUpdate();
    }

}
