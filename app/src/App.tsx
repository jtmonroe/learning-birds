import { Component } from 'solid-js';

import styles from './App.module.css';
import { RenderSim, RenderSimProps } from './RenderSim';



const App: Component = () => {
  const ids: RenderSimProps = {
    generation_id: 'generation_stat',
    previous_fitness_id: 'previous_fitness_stat'
  };

  return (
    <div class={styles.App}>
      < RenderSim {...ids} />
    </div>
  );

};

export default App;

