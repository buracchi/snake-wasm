export class CanvasContainer {
    container: HTMLElement;
    canvas: HTMLCanvasElement;
    context: CanvasRenderingContext2D;

    private resize_observer: ResizeObserver;
    
    constructor() {
        this.container = document.createElement('main');
        this.canvas = document.createElement('canvas');
        this.container.appendChild(this.canvas);
        this.context = this.canvas.getContext('2d');
    }

    onContainerResize(callback: () => void) {
        this.resize_observer = new ResizeObserver((entries: ResizeObserverEntry[]) => entries.forEach(_ => callback()));
        this.resize_observer.observe(this.container);
    }

    resetCanvas() {
        this.container.removeChild(this.canvas);
        this.canvas = document.createElement('canvas');
        this.container.appendChild(this.canvas);
        this.context = this.canvas.getContext('2d');
    }

}
