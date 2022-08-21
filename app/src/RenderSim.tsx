import { createSignal, onCleanup, onMount } from 'solid-js';
import styles from './App.module.css';
import { SimDrawer } from './SimDrawer';

export interface RenderSimProps {
    generation_id: string;
    previous_fitness_id: string;
};

// TBH This function is a TERRIBLE plan and very brittle
function getStyleAttribute(object_id: string, style_id: string): string {
    for (const sheet of document.styleSheets) {
        for (const rule of sheet.cssRules) {
            const styleRule = rule as CSSStyleRule;
            if (styleRule.selectorText == `.${object_id}`) {
                return styleRule.style.getPropertyValue(style_id)
            }
        }
    }
    return ""
}


export const RenderSim = (props: RenderSimProps) => {
    const [isBird, _] = createSignal(true);

    getStyleAttribute(styles.bird, "outline-color")
    const simDrawer = new SimDrawer(
        props.previous_fitness_id,
        props.generation_id,
        getStyleAttribute(styles.bird, "outline-color"),
        getStyleAttribute(styles.food, "outline-color"),
    );

    let canvasRef: HTMLCanvasElement | undefined = undefined;
    let frame: number;

    const draw = (_: number) => {
        const ctx = canvasRef?.getContext('2d') ?? null;

        if (ctx && canvasRef && isBird()) {
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


    const Age = () => { // not-reactive, constant update
        const [age, setAge] = createSignal(0);
        const interval = setInterval(
            () => setAge(simDrawer.age())
        );
        onCleanup(() => clearInterval(interval));
        return <div>{age()}</div>;
    }

    const Fitness = () => { // not reactive, constant update
        const [fitness, setFitness] = createSignal(0);
        const interval = setInterval(
            () => setFitness(simDrawer.average_fitness()),
            10
        );
        onCleanup(() => clearInterval(interval));
        return <div>{fitness().toFixed(3)}</div>;
    }

    const statsTable = [
        { name: "Generation", id: props.generation_id, default: "0" },
        { name: "Age", fun: Age },
        { name: "Current Fitness", fun: Fitness },
        { name: "Previous Fitness", id: props.previous_fitness_id, default: "0.0" }
    ]

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
                        {statsTable.map(row => {
                            return (
                                <tr>
                                    <th class={styles.table_label}>{row.name}</th>
                                    {("id" in row) ? <td id={row.id}>{row.default!}</td> : <td> {row.fun()} </td>}
                                </tr>
                            )
                        })}
                    </table>
                </div>
            </div>
        </div>


    )
}
