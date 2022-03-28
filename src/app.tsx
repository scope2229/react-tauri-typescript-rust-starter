import { useCallback, useState } from 'react';

const App = () => {
  const [count, setCount] = useState(0);
  const increment = useCallback(() => setCount(count + 1), [count]);
  const decrement = useCallback(() => setCount(count - 1), [count]);

  return (
    <>
      <h1>Count: {count}</h1>
      <button onClick={increment}>Increment</button>
      <button onClick={decrement}>Decrement</button>
    </>
  )
};

export default App;