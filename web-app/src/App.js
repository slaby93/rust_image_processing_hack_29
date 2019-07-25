import React, { Component } from 'react'
import Spinner from './components/Spinner'
import Images from './components/Images'
import Buttons from './components/Buttons'
import './App.css'

// function App() {
//   return (
//     <div className="App">
//       <header className="App-header">
//         <Spinner />
//       </header>
//     </div>
//   );
// }

export default class App extends Component {
  state = {
    uploading: false,
    images: []
  }

  onChange = e => {
    const files = Array.from(e.target.files)
    this.setState({ uploading: true })
    
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

    return (
      <div className="App">
        <header className="App-header">
          <div className='buttons'>
            {content()}
          </div>
        </header>
      </div>
    )
  }
}
