import { Component } from 'solid-js';

import styles from './App.module.css';
import { RenderSim } from './RenderSim';


const App: Component = () => {
  return (
    <div class={styles.App}>
      <div class={styles.content_row}>
        <RenderSim/>
        <div class={styles.content_col}>
          <h1 class={styles.title}>Content</h1>
          <p>A lot of random content</p>
        </div>
      </div>

    </div>
  );
};

export default App;
