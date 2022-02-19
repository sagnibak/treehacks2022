import './Editor.css';
import AceEditor from 'react-ace';

function Editor() {
    let handleInput = (value) => {
        console.log(value);
    }

    return (
        <AceEditor mode="java" theme="github" name="editor" onChange={handleInput} />
    )
}

export default Editor;