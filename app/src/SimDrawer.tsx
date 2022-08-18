import * as sim from 'lib-simulation-wasm';


export class SimDrawer {
    private simulation = new sim.Simulation();

    age(): number {
        return this.simulation.age();
    }

    redraw(canvasContext: CanvasRenderingContext2D, viewportHeight: number, viewportWidth: number) {
        console.log("redraw");
        canvasContext.clearRect(0, 0, viewportWidth, viewportHeight);
        this.simulation.step();

        const world = this.simulation.world();

        for (const food of world.food) {
            drawCircle(
                canvasContext,
                food.x * viewportWidth,
                food.y * viewportHeight,
                (0.01 / 2.0) * viewportWidth
            );
        }

        for (const animal of world.animals) {
            drawTriangle(
                canvasContext,
                animal.x * viewportWidth,
                animal.y * viewportHeight,
                0.01 * viewportWidth,
                animal.rotation
            );
        }

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