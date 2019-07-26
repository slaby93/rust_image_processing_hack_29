export default (file, onComplete, onError) => new Promise((resolve) => {
    {
        const reader = new FileReader()
        reader.readAsDataURL(file)
        reader.onload = () => {
            resolve(reader.result)
        }
    }
})