import React from 'react'
import ACTIONS from './../actions.json'

export default ({ transformImage }) => {
    return (
        <div>
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
