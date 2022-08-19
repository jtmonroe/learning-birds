import { createSignal, onCleanup, onMount } from 'solid-js';
import styles from './App.module.css';
import { SimDrawer } from './SimDrawer';

export interface RenderSimProps { };

export const RenderSim = (props: RenderSimProps) => {
    const [isBird, _] = createSignal(true);
    const simDrawer = new SimDrawer();

    let canvasRef: HTMLCanvasElement | undefined = undefined;
    let frame: number;

    const draw = (time: number) => {
        const ctx = canvasRef?.getContext('2d') ?? null;

        if (ctx && canvasRef && isBird()) {
            ctx.fillStyle = 'rgb(0, 0, 0)';
            simDrawer.redraw(ctx, canvasRef.height, canvasRef.width);
            frame = requestAnimationFrame(draw);
        }
    };

    onMount(() => {
        const viewportScale = window.devicePixelRatio || 1;
        canvasRef!.width = canvasRef!.clientWidth * viewportScale;
        canvasRef!.height = canvasRef!.clientHeight * viewportScale;

        frame = requestAnimationFrame(draw);
        onCleanup(() => cancelAnimationFrame(frame));
    });

    const Age = () => {
        const [age, setAge] = createSignal(0);
        const interval = setInterval(
            () => setAge(simDrawer.age())
        );
        onCleanup(() => clearInterval(interval));
        return <div>{age()}</div>;
    }

    const Fitness = () => {
        const [fitness, setFitness] = createSignal(0);
        const interval = setInterval(
            () => setFitness(simDrawer.average_fitness()),
            10
        );
        onCleanup(() => clearInterval(interval));
        return <div>{fitness().toPrecision(4)}</div>;
    }

    const Generation = () => {
        const [generation, setGeneration] = createSignal(0);
        const interval = setInterval(
            () => setGeneration(simDrawer.current_generation()),
        );
        onCleanup(() => clearInterval(interval));
        return <div>{generation()}</div>;
    }

    const Prev_Fitness = () => {
        const [generation, setGeneration] = createSignal(0);
        const interval = setInterval(
            () => setGeneration(simDrawer.previous_fitness()),
            10
        );
        onCleanup(() => clearInterval(interval));
        return <div>{generation().toPrecision(4)}</div>;
    }


    return (
        <div class={styles.content_row}>
            <div class={styles.content_col}>
                <div class={styles.render}>
                    <canvas ref={canvasRef} class={styles.animation}></canvas>
                </div>
            </div>
            <div class={styles.content_col}>
                <div class={styles.stats}>
                    <h1 class={styles.title}>Learning Birds</h1>

                    <h2>Description</h2>
                    <p>Each bird contains both a sensor and a brain neural network. While this network could be refined through <a href="https://en.wikipedia.org/wiki/Backpropagation">Backpropagation</a>, it is instead refined through a genetic algorithm.</p>

                    <h2>Statistics</h2>

                    <table>
                        <thead>
                            <tr>
                                <th>Generation</th>
                                <th>Age</th>
                                <th>Current Fitness</th>
                                <th>Previous Fitness</th>
                            </tr>
                        </thead>
                        <tbody>
                            <tr>
                                <td>{Generation()}</td>
                                <td>{Age()}</td>
                                <td>{Fitness()}</td>
                                <td>{Prev_Fitness()}</td>
                            </tr>
                        </tbody>

                    </table>
                </div>
            </div>
        </div>


    )
}
