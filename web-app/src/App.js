import React, { Component } from 'react'
import Spinner from './components/Spinner'
import Images from './components/Images'
import Buttons from './components/Buttons'
import Transforms from './components/Transforms'
import Base64 from './utils/Base64'
import { faUndo, faCameraRetro } from '@fortawesome/free-solid-svg-icons'
import './App.css'

let wasm

(async () => {
  try {
    wasm = await import('bindings');
  } finally {
    console.log('Loaded')
  }
})()

export default class App extends Component {
  state = {
    uploading: false,
    images: []
  }

  onChange = e => {
    const files = Array.from(e.target.files)
    this.setState({ uploading: true })
    
    console.log(Base64.toBase64(files[0], result => {
      console.log(result)
    }, error => {
      console.e(error)
    }))

    setTimeout(() => {
      let images = files.map(file => {
        return {
          public_id: file.lastModified,
          secure_url: URL.createObjectURL(file)
        }
      })

      console.log(images)

      this.setState({
        uploading: false,
        images
      })
    }, 500)
  }

  removeImage = id => {
    this.setState({
      images: this.state.images.filter(image => image.public_id !== id)
    })
  }

  render() {
    const { uploading, images } = this.state
    const content = () => {
      switch(true) {
        case uploading:
          return <Spinner />
        case images.length > 0:
          return <Images images={images} removeImage={this.removeImage} />
        default:
          return <Buttons onChange={this.onChange} />
      }
    }

    const transforms = [
      {
        icon: faUndo,
        title: 'Rotate',
        onClick: () => { wasm.greet("YOYOYO") }
      },
      {
        icon: faCameraRetro,
        title: 'Grayscale',
        onClick: () => { console.log("Done") }
      }
    ]

    return (
      <div className="App">
        <header className="App-header">
          <div className='buttons'>
            {content()}
          </div>
          <Transforms transforms={transforms} />
        </header>
      </div>
    )
  }
}
