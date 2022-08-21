import init, { Simulation } from 'lib-simulation-wasm';

await init();

export class SimDrawer {
    private simulation: Simulation;
    private bird_color: string;
    private food_color: string;

    constructor(
        previous_fitness_id: string,
        generation_id: string,
        bird_color: string,
        food_color: string
    ) {
        this.simulation = new Simulation(generation_id, previous_fitness_id)
        this.bird_color = bird_color;
        this.food_color = food_color;
    }

    age = (): number => this.simulation.age();

    average_fitness = (): number => this.simulation.average_fitness();

    redraw(canvasContext: CanvasRenderingContext2D, viewportHeight: number, viewportWidth: number) {
        canvasContext.clearRect(0, 0, viewportWidth, viewportHeight);
        this.simulation.step();

        const world = this.simulation.world();

        for (const food of world.food) {
            drawCircle(
                canvasContext,
                food.x * viewportWidth,
                food.y * viewportHeight,
                (0.01 / 2.0) * viewportWidth,
                this.food_color
            );
        }

        for (const animal of world.animals) {
            drawTriangle(
                canvasContext,
                animal.x * viewportWidth,
                animal.y * viewportHeight,
                0.01 * viewportWidth,
                animal.rotation,
                this.bird_color
            );
        }

    }

}

// birds
function drawTriangle(ctx: CanvasRenderingContext2D, x: number, y: number, size: number, rotation: number, color: string) {
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

    ctx.strokeStyle = color; // A nice white color
    ctx.stroke();

};

function drawCircle(ctx: CanvasRenderingContext2D, x: number, y: number, radius: number, color: string) {
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

    ctx.fillStyle = color; // A nice green color
    ctx.fill();
};