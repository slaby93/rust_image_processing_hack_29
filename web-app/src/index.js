import React from 'react';
import ReactDOM from 'react-dom';
import './index.css';
import App from './App';
import * as serviceWorker from './serviceWorker';

(async () => {
    try {
      const wasm = await import('bindings');
      ReactDOM.render(<App wasm={wasm} />, document.getElementById('root'));
      console.log('WASM module has been loaded')
    } catch(error) {
      alert('WASM module cant be loaded. Checkout console for details')
      throw error
    }
  })()

// If you want your app to work offline and load faster, you can change
// unregister() to register() below. Note this comes with some pitfalls.
// Learn more about service workers: https://bit.ly/CRA-PWA
serviceWorker.unregister();
