import React, { useRef } from 'react'
import ACTIONS from './../actions.json'
import './ActionBar.css'

export default ({ transformImage }) => {
    const wR1 = useRef()
    const wH2 = useRef()
    const wR3 = useRef()
    const wH4 = useRef()
    const w5 = useRef()
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
                <input ref={wR3}/>
                <input ref={wH4}/>
                <button onClick={() => transformImage('best_fit_resize', wR3.current.value, wH4.current.value)}>
                    Best fit resize        
                </button>
            </div>
            <div className="resize">
                <input ref={wR1}/>
                <input ref={wH2}/>
                <button onClick={() => transformImage('exact_resize', wR1.current.value, wH2.current.value)}>
                    Resize        
                </button>
            </div>
                <div className="resize">
                <input ref={w5}/>
                <button onClick={() => transformImage('rotate', w5.current.value)}>
                    Rotate
                </button>
            </div>
        </div>
    )
}
