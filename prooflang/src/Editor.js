import './Editor.css';
// import AceEditor from 'react-ace';
import init from 'prooflang-wasm';
import React from 'react';

function Editor() {
    const [wasm, setWasm] = React.useState(null);
    const [env, setEnv] = React.useState(null);
    const [output, setOutput] = React.useState([]);

    React.useEffect(() => {
        if ( !wasm ) init().then(wasm => setWasm(wasm));
        else wasm.greet()
    }, [wasm]);
    

    let handleInput = (value) => {
        console.log(value);
        setOutput(output.concat(">>> " + value, wasm.interpret(value)))
        setEnv(wasm.get_env());
    }

    const handleKeyDown = (event) => {
        if (event.key === 'Enter') {
            handleInput(event.target.textContent);
            event.target.textContent = '';
        }
    }

    React.useEffect(() => {
        setOutput(['test', 'test2']);
    }, []);

    return (
        <div className='editor'>
            {output.map((value, index) => (
                <span className='code' key={"output" + index}>{value}</span>
            ))}
            <div className='codeRow'>
                <span>>>> </span>
                <span className='code' id="input" contentEditable="plaintext-only" onKeyDown={handleKeyDown}></span>
            </div>
        </div>
        // <AceEditor mode="java" theme="github" name="editor" onChange={handleInput} />
    )
}

export default Editor;