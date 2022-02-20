import './Editor.css';
import init from 'prooflang-wasm';
import React from 'react';
import { interpret, get_env } from 'prooflang-wasm';

function Editor() {
    const [wasm, setWasm] = React.useState(null);
    const [env, setEnv] = React.useState(null);
    const [output, setOutput] = React.useState([]);

    React.useEffect(() => {
        if ( !wasm ) init().then(wasm => setWasm(wasm));
        else {
            wasm.greet()
        }
    }, [wasm]);
    

    let handleInput = (value) => {
        console.log(value);
        setOutput(output.concat(">>> " + value, interpret(value)))
        setEnv(get_env());
    }

    const handleKeyDown = (event) => {
        if (event.key === 'Enter' && wasm) {
            handleInput(event.target.textContent);
            event.target.textContent = '';
        }
    }

    return (
        <div className='editor'>
            {output.map((value, index) => (
                <span className='code' key={"output" + index}>{value}</span>
            ))}
            <div className='codeRow'>
                <span>{'>>> '}</span>
                <span className='code' id="input" contentEditable="plaintext-only" onKeyDown={handleKeyDown}></span>
            </div>
        </div>
    )
}

export default Editor;