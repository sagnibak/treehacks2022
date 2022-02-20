import './Display.css';
import randomColor from 'randomcolor';
import React from 'react';

function Display({wasm, env, types}) {
    const [colorMap, setColorMap] = React.useState({null: '#000000'});
    const [envColorMap, setEnvColorMap] = React.useState({null: '#000000'});
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
    
    React.useEffect(() => {
        env && env.map(t => {
            let name = t[0];
            if (!envColorMap[name]) {
                envColorMap[name] = randomColor({luminosity: 'dark'});
                setEnvColorMap(envColorMap);
            }
    })}, [env]);

    return (
        <div className="Display">
            <h1>Display</h1>
            {/* <p>types: {types}</p> */}
            {/* <p>objects: {env && env.map(t => (<span>{t[0]}</span>))}</p> */}
            <h2>Types</h2>
            <div className='type-list'> 
                {types && types.map(t => (
                <div className='type' style={{backgroundColor:colorMap[t[0]]}}>
                    <span className='type-name'>{t[0]}</span>  
                    {/* color={(brightness > 125) ? 'black' : 'white'} */}
                    <div className='unit-selection'>
                        {t[1].map(unit => <span className='unit'>{unit}</span>)}
                    </div>
                </div>))}
            </div>
            <h2>Objects</h2>
            <div className='env-list'> 
                {env && env.map(t => (
                <div className='env' style={{color:envColorMap[t[0]]}}>{t[0] + " : " + t[1] + " -> " + t[2]}
                </div>))}
            </div>
        </div>
    )
}

export default Display;