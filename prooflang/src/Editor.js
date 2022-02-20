import './Editor.css';
import AceEditor from 'react-ace';
import * as wasm from 'prooflang-wasm';

function Editor() {
    let handleInput = (value) => {
        console.log(value);
    }


    wasm.greet();

    return (
        <AceEditor mode="java" theme="github" name="editor" onChange={handleInput} />
    )
}

export default Editor;