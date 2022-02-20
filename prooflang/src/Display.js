import './Display.css';
import randomColor from 'randomcolor';
import React from 'react';

function Display({wasm, env, types}) {
    const [colorMap, setColorMap] = React.useState({null: '#000000'});
    window.types = types;
    window.env = env;
    
    React.useEffect(() => {
        types && types.map(t => {
            let name = t[0];
            if (!colorMap[name]) {
                colorMap[name] = randomColor({luminosity: 'dark'});
                setColorMap(colorMap);
            }
    })}, [types]);

    return (
        <div className="Display">
            <h1>Display</h1>
            <p>types: {types}</p>
            <p> Types </p>
            <div className='type-list'> 
                {types && types.map(t => (
                <div className='type' style={{backgroundColor:colorMap[t[0]]}}>
                    <span className='type-name'>{t[0]}</span>  
                    <div className='unit-selection'>
                        {t[1].map(unit => <span className='unit'>{unit}</span>)}
                    </div>
                </div>))}
            </div>
            
        </div>
    )
}

export default Display;