import './Display.css';

function Display({wasm, env, types}) {
    window.types = types;
    window.env = env;
    return (
        <div className="Display">
            <h1>Display</h1>
            <p>types: {types}</p>
            <div className='type-list'> 
                <div className='type'>
                    Bool
                    <div className='unit-selection'>
                        <span className='unit'>True</span>
                        <span className='unit'>False</span>
                    </div>
                </div>
            </div>
            
        </div>
    )
}

export default Display;