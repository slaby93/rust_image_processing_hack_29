import React from 'react'
import { FontAwesomeIcon } from '@fortawesome/react-fontawesome'

export default props => {
    const transforms = props.transforms.map(transform => (
        <div key={transform.title} className='transformItem'>
            <div className='transform' onClick={() => transform.onClick()}>
                <FontAwesomeIcon icon={transform.icon} size='1x' color='black' />
            </div>
            <a>{transform.title}</a>
        </div>
    ))

    return (
        <div className='transformContainer'>
            { transforms }
        </div>
    )
}
