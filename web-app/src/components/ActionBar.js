import React from 'react'
import ACTIONS from './../actions.json'
import './ActionBar.css'

export default ({ transformImage }) => {
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
        </div>
    )
}
