export class Scoreboard {
    currentScore: number;
    bestScore: number;
    htmlElement: HTMLElement;
    currScoreParagraph: HTMLParagraphElement;
    bestScoreParagraph: HTMLParagraphElement;
    currScoreSpan: HTMLSpanElement;
    bestScoreSpan: HTMLSpanElement;

    constructor() {
        this.currentScore = 0;
        this.bestScore = 0;
        this.htmlElement = document.createElement('header');
        this.currScoreParagraph = document.createElement('p');
        this.bestScoreParagraph = document.createElement('p');
        this.currScoreParagraph.innerText = "CURRENT: ";
        this.bestScoreParagraph.innerText = "BEST: ";
        this.currScoreSpan = document.createElement('span');
        this.bestScoreSpan = document.createElement('span');
        this.currScoreParagraph.appendChild(this.currScoreSpan);
        this.bestScoreParagraph.appendChild(this.bestScoreSpan);
        this.htmlElement.appendChild(this.currScoreParagraph);
        this.htmlElement.appendChild(this.bestScoreParagraph);
        this.currScoreSpan.id = "current-score";
        this.bestScoreSpan.id = "best-score";
        this.currScoreSpan.innerText = this.currentScore.toString();
        this.bestScoreSpan.innerText = this.bestScore.toString();
    }

    setCurrentScore(score: number) {
        this.currentScore = score;
        this.currScoreSpan.innerText = this.currentScore.toString();
    }

    setBestScore(score: number) {
        this.bestScore = score;
        this.bestScoreSpan.innerText = this.bestScore.toString();
    }
}
