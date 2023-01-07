import { CanvasResizeEvent } from "./canvas-container";

export class Scoreboard {
    htmlElement: HTMLElement;
    currentScoreElement: HTMLParagraphElement;
    bestScoreElement: HTMLParagraphElement;
    currScoreSpan: HTMLSpanElement;
    bestScoreSpan: HTMLSpanElement;

    constructor() {
        this.htmlElement = document.createElement('header');
        this.htmlElement.id = 'scoreboard';
        this.currentScoreElement = document.createElement('p');
        this.bestScoreElement = document.createElement('p');
        this.currentScoreElement.innerText = "CURRENT: ";
        this.bestScoreElement.innerText = "BEST: ";
        this.currScoreSpan = document.createElement('span');
        this.bestScoreSpan = document.createElement('span');
        this.currentScoreElement.appendChild(this.currScoreSpan);
        this.bestScoreElement.appendChild(this.bestScoreSpan);
        this.htmlElement.appendChild(this.currentScoreElement);
        this.htmlElement.appendChild(this.bestScoreElement);
        this.currScoreSpan.id = "current-score";
        this.bestScoreSpan.id = "best-score";
        document.addEventListener('canvas-resize', (event: CanvasResizeEvent) => {
            this.htmlElement.style.width = `${event.width}px`;
        });
    }

    setCurrentScore(score: number) {
        this.currScoreSpan.innerText = score.toString();
    }

    setBestScore(score: number) {
        this.bestScoreSpan.innerText = score.toString();
    }

}
