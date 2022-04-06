// import * as wasm from "hello-wasm-pack";
// wasm.greet();

import * as wasm from 'wasm-player'

wasm.greet().then(txt => {
    console.log(11, txt)
    const pre = document.createElement('pre')
    pre.style = 'font-size: 12px;'
    pre.innerText = txt
    document.body.appendChild(pre)
}).catch(e => {
    console.log(12, e)
})

