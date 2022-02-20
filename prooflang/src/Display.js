import './Display.css';
import randomColor from 'randomcolor';
import React from 'react';

function Display({wasm, env, types}) {
    const [colorMap, setColorMap] = React.useState({null: '#000000'});
    window.types = types;
    window.env = env;
    
    if (!("Bool" in colorMap)) {
        setColorMap({...colorMap, "Bool":randomColor()});
    }
    if (!("Singleton" in colorMap)) {
        setColorMap({...colorMap, "Singleton":randomColor()});
    }
    return (
        <div className="Display">
            <h1>Display</h1>
            <p>types: {types}</p>
            <p> Types </p>
            <div className='type-list'> 
                <div className='type' style={{backgroundColor:colorMap["Bool"]}}>
                    <span className='type-name'>Bool</span>  
                    <div className='unit-selection'>
                        <span className='unit'>True</span>
                        <span className='unit'>False</span>
                    </div>
                </div>

                <div className='type'>
                    <span className='type-name'>Bool</span>  
                    <div className='unit-selection'>
                        <span className='unit'>True</span>
                        <span className='unit'>False</span>
                    </div>
                </div>

                <div className='type'>
                    <span className='type-name'>Bool</span>  
                    <div className='unit-selection'>
                        <span className='unit'>True</span>
                        <span className='unit'>False</span>
                    </div>
                </div>

                <div className='type'>
                    <span className='type-name'>Bool</span>  
                    <div className='unit-selection'>
                        <span className='unit'>True</span>
                        <span className='unit'>False</span>
                    </div>
                </div>

                <div className='type'>
                    <span className='type-name'>Bool</span>  
                    <div className='unit-selection'>
                        <span className='unit'>True</span>
                        <span className='unit'>False</span>
                    </div>
                </div>

                <div className='type'>
                    <span>Bool</span>  
                    <div className='unit-selection'>
                        <span className='unit'>True</span>
                        <span className='unit'>False</span>
                    </div>
                </div>

                <div className='type' style={{backgroundColor:colorMap["Singleton"]}}>
                    <span>Singleton</span>  
                    <div className='unit-selection'>
                        <span className='unit'>One</span>
                    </div>
                </div>
            </div>
            
        </div>
    )
}

export default Display;