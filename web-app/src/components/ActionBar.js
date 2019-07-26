import React, { useRef } from 'react'
import ACTIONS from './../actions.json'
import './ActionBar.css'

export default ({ transformImage }) => {
    const wR = useRef()
    const wH = useRef()
    return (
        <div className="ActionBar">
            {
                Object.entries(ACTIONS).map(([name, meta]) => {
                    return (
                        <button onClick={() => transformImage(meta.wasm_function_name)} key={name}>
                            {name}
                        </button>
                    )
                })
            }
            <div className="resize">
                <input ref={wR}/>
                <input ref={wH}/>
                <button onClick={() => transformImage('exact_resize', wR.current.value, wH.current.value)}>
                    Resize        
                </button>
            </div>
        </div>
    )
}
