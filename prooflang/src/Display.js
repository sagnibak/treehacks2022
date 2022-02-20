import './Display.css';

function Display({wasm, env, types}) {
    window.types = types;
    return (
        <div className="Display">
            <h1>Display</h1>
            <p>types: {types}</p>
        </div>
    )
}

export default Display;