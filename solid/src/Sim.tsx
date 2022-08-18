import * as sim from 'lib-simulation-wasm';


export class Sim {
    private simulation = new sim.Simulation();
    private context: CanvasRenderingContext2D;
    private viewportWidth: number;
    private viewportHeight: number;
    private canvas: HTMLCanvasElement;

    constructor(canvas: HTMLCanvasElement) {
        this.canvas = canvas;
        this.viewportWidth = canvas.clientWidth;
        this.viewportHeight = canvas.clientHeight;

        let viewportScale = window.devicePixelRatio || 1;
        canvas.width = this.viewportWidth * viewportScale;
        canvas.height = this.viewportHeight * viewportScale;

        this.context = canvas.getContext('2d')!;
        this.context.fillStyle = 'rgb(0, 0, 0)';
    }

    redraw() {
        this.context.clearRect(0, 0, this.viewportWidth, this.viewportHeight);

        this.simulation.step();
    
        const world = this.simulation.world();
    
        for (const food of world.food) {
            drawCircle(
                this.context,
                food.x * this.viewportWidth,
                food.y * this.viewportHeight,
                (0.01 / 2.0) * this.viewportWidth
            );
        }
    
        for (const animal of world.animals) {
            drawTriangle(
                this.context,
                animal.x * this.viewportWidth,
                animal.y * this.viewportHeight,
                0.01 * this.viewportWidth,
                animal.rotation
            );
        }
    }

    Animate = () => {
        this.redraw();
        requestAnimationFrame(this.Animate);
    }

    public animate(): HTMLCanvasElement {
        this.Animate();
        return this.canvas;
    }

}

function drawTriangle(ctx: CanvasRenderingContext2D, x: number, y: number, size: number, rotation: number) {
    ctx.beginPath();

    ctx.moveTo(
        x + Math.cos(rotation) * size * 1.5,
        y + Math.sin(rotation) * size * 1.5,
    );


    ctx.lineTo(
        x + Math.cos(rotation + 2.0 / 3.0 * Math.PI) * size,
        y + Math.sin(rotation + 2.0 / 3.0 * Math.PI) * size,
    );

    ctx.lineTo(
        x + Math.cos(rotation + 4.0 / 3.0 * Math.PI) * size,
        y + Math.sin(rotation + 4.0 / 3.0 * Math.PI) * size,
    );

    ctx.lineTo(
        x + Math.cos(rotation) * size * 1.5,
        y + Math.sin(rotation) * size * 1.5,
    );

    ctx.strokeStyle = 'rgb(255, 255, 255)'; // A nice white color
    ctx.stroke();

};

function drawCircle(ctx: CanvasRenderingContext2D, x: number, y: number, radius: number) {
    ctx.beginPath();

    // ---
    // | Circle's center.
    // ----- v -v
    ctx.arc(x, y, radius, 0, 2.0 * Math.PI);
    // ------------------- ^ -^-----------^
    // | Range at which the circle starts and ends, in radians.
    // |
    // | By manipulating these two parameters you can e.g. draw
    // | only half of a circle, Pac-Man style.
    // ---

    ctx.fillStyle = 'rgb(0, 255, 128)'; // A nice green color
    ctx.fill();
};