import React, { useState, useCallback } from 'react'
import ActionBar from './components/ActionBar'
import Spinner from './components/Spinner'
import ImageDisplay from './components/ImageDisplay'
import UploadImageForm from './components/UploadImageForm'
import file_to_base64 from './utils/file_to_base64'
import './App.css'

const App = ({ wasm }) => {
  const [ imageBase64, setImageBase64 ] = useState(null)
  const [ uploading, setUploading ] = useState(false)
  const onUpload = useCallback(async ({ target: { files: [ file ] }}) => {
    setUploading(true)
    const parsedBase64File = await file_to_base64(file)
    console.log({ parsedBase64File })
    setImageBase64(parsedBase64File)
    setUploading(false)
  })
  const onImageTransform = useCallback(wasm_function_name => {
    setUploading(true)
    // TODO: FINISH THIS
    const newImage = wasm.load_image(imageBase64)
    setImageBase64(newImage)
    setUploading(false)
  })

  return (
    <div className="App">
      <header className="App-header">
        {uploading && <Spinner/> }
        {!imageBase64 && (
          <UploadImageForm onUpload={onUpload} />
        )}
        {imageBase64 && (
          <div className='main-content'>
            <ImageDisplay imageBase64={imageBase64} />
            <ActionBar transformImage={onImageTransform} />
          </div>
        )}
        
      </header>
    </div>
  )
}

export default App

// export default class App extends Component {
//   state = {
//     uploading: false,
//     image: null
//   }

//   onChange = e => {
//     const files = Array.from(e.target.files)
//     this.setState({ uploading: true })
    
//     console.log(Base64.toBase64(files[0], result => {
//       console.log(result)
//     }, error => {
//       console.e(error)
//     }))

//     setTimeout(() => {
//       let images = files.map(file => {
//         return {
//           public_id: file.lastModified,
//           secure_url: URL.createObjectURL(file)
//         }
//       })

//       console.log(images)

//       this.setState({
//         uploading: false,
//         images
//       })
//     }, 500)
//   }

//   removeImage = id => {
//     this.setState({
//       images: this.state.images.filter(image => image.public_id !== id)
//     })
//   }

//   render() {
//     const { wasm } = this.props
//     const { uploading, image } = this.state

//     const transforms = [
//       {
//         icon: faUndo,
//         title: 'Rotate',
//         onClick: () => { wasm.greet("YOYOYO") }
//       },
//       {
//         icon: faCameraRetro,
//         title: 'Grayscale',
//         onClick: () => { console.log("Done") }
//       }
//     ]

//     return (
//       <div className="App">
//         <header className="App-header">
//           <div className='buttons'>
//             {content()}
//           </div>
//           <Transforms transforms={transforms} />
//         </header>
//       </div>
//     )
//   }
// }
