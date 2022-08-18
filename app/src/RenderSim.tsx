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

    return (
        <div class={styles.render}>
            <canvas ref={canvasRef} class={styles.animation}></canvas>
        </div>
    )
}
