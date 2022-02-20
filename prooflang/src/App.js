import logo from './logo.svg';
import './App.css';
import Editor from './Editor.js';
import Display from './Display.js';

function App() {
  return (
    <div className="App">
      <div className="App-header">
        <h1>ProofLang</h1>
      </div>
      <div className='App-body'>
        <Editor />
        <Display />
      </div>
    </div>
  );
}

export default App;
