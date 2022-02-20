import './App.css';
import Editor from './Editor.js';
import Display from './Display.js';
import React from 'react';
import init from 'prooflang-wasm';

function App() {
  const [wasm, setWasm] = React.useState(null);
  const [env, setEnv] = React.useState(null);
  const [types, setTypes] = React.useState(null);

  React.useEffect(() => {
    if ( !wasm ) init().then(wasm => setWasm(wasm));
    else {
        wasm.greet();
    }
  }, [wasm]);

  return (
    <div className="App">
      <div className="App-header">
        <h1>ProofLang</h1>
      </div>
      <div className='App-body'>
        <Editor wasm={wasm} setEnv={setEnv} setTypes={setTypes}/>
        <Display wasm={wasm} env={env} types={types}/>
      </div>
    </div>
  );
}

export default App;
