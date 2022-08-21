import { Component } from 'solid-js';

import styles from './App.module.css';
import { RenderSim, RenderSimProps } from './RenderSim';



const App: Component = () => {
  const ids: RenderSimProps = {
    generation_id: 'a',
    previous_fitness_id: 'b'
  }

  return (
    <div class={styles.App}>
      < RenderSim {...ids} />
    </div>
  );

};

export default App;

