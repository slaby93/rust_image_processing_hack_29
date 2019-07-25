export default {
    toBase64: (file, onComplete, onError) => {
        const reader = new FileReader()
        reader.readAsDataURL(file)
        reader.onerror = onError
        reader.onload = () => {
            onComplete(reader.result)
        }
    }
}
