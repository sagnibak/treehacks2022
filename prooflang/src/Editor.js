import './Editor.css';
import React from 'react';
import { interpret, get_env, get_types } from 'prooflang-wasm';

function Editor({wasm, setEnv, setTypes}) {
    const [output, setOutput] = React.useState([]);

    

    let handleInput = (value) => {
        console.log(value);
        setOutput(output.concat(">>> " + value, interpret(value)))
        setEnv(JSON.parse(get_env()));
        setTypes(JSON.parse(get_types()));
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