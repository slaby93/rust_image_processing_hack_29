import * as wasm from "wasm-bindings";

document.getElementById("submit-me")
    .addEventListener("click", () => {
        const text = document.getElementById("test").value
        wasm.greet(text);
    })