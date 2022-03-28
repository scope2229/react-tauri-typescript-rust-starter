import React, { FunctionComponent, useCallback, useState } from 'react';
import { invoke } from '@tauri-apps/api';

const App: FunctionComponent = () => {
  // For Persistent state use Rust to store the state in a file
  const [count, setCount] = useState(0);

  const increment = useCallback(() => {
    invoke('increment', { argument: count })
      .then((response: any) => {
        console.log("res from rust", response);
        setCount(response);
      })
      .catch((error: Error) => {
        console.error(error);
      });
  }, [count]);

  const decrement = useCallback(() => {
    invoke("decrement", { argument: count })
      .then((response: any) => {
        console.log("res from rust", response);
        setCount(response);
      })
      .catch((error: Error) => {
        console.error(error);
      });
  }, [count]);

  return (
    <>
      <h1>Count: {count}</h1>
      <button onClick={increment}>Increment</button>
      <button onClick={decrement}>Decrement</button>
    </>
  )
};

export default App;