import { Component } from 'solid-js';

import styles from './App.module.css';
import { RenderSim } from './RenderSim';


const App: Component = () => {
  return (
    <div class={styles.App}>
      < RenderSim />
    </div>
  );
};

export default App;
