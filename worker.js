!function(e){self.webpackChunk=function(n,o){for(var r in o)e[r]=o[r];for(;n.length;)t[n.pop()]=1};var n={},t={0:1},o={};var r={2:function(){return{"./rchess_wasm":{__wbg_log_06b108a730166efb:function(e,t){return n[1].exports.__wbg_log_06b108a730166efb(e,t)},__wbg_error_cc95a3d302735ca3:function(e,t){return n[1].exports.__wbg_error_cc95a3d302735ca3(e,t)},__wbindgen_throw:function(e,t){return n[1].exports.__wbindgen_throw(e,t)}}}}};function i(t){if(n[t])return n[t].exports;var o=n[t]={i:t,l:!1,exports:{}};return e[t].call(o.exports,o,o.exports,i),o.l=!0,o.exports}i.e=function(e){var n=[];return n.push(Promise.resolve().then((function(){t[e]||importScripts(e+".worker.js")}))),({1:[2]}[e]||[]).forEach((function(e){var t=o[e];if(t)n.push(t);else{var s,a=r[e](),u=fetch(i.p+""+{2:"dc4f43f1177b5fa30d2c"}[e]+".module.wasm");if(a instanceof Promise&&"function"==typeof WebAssembly.compileStreaming)s=Promise.all([WebAssembly.compileStreaming(u),a]).then((function(e){return WebAssembly.instantiate(e[0],e[1])}));else if("function"==typeof WebAssembly.instantiateStreaming)s=WebAssembly.instantiateStreaming(u,a);else{s=u.then((function(e){return e.arrayBuffer()})).then((function(e){return WebAssembly.instantiate(e,a)}))}n.push(o[e]=s.then((function(n){return i.w[e]=(n.instance||n).exports})))}})),Promise.all(n)},i.m=e,i.c=n,i.d=function(e,n,t){i.o(e,n)||Object.defineProperty(e,n,{enumerable:!0,get:t})},i.r=function(e){"undefined"!=typeof Symbol&&Symbol.toStringTag&&Object.defineProperty(e,Symbol.toStringTag,{value:"Module"}),Object.defineProperty(e,"__esModule",{value:!0})},i.t=function(e,n){if(1&n&&(e=i(e)),8&n)return e;if(4&n&&"object"==typeof e&&e&&e.__esModule)return e;var t=Object.create(null);if(i.r(t),Object.defineProperty(t,"default",{enumerable:!0,value:e}),2&n&&"string"!=typeof e)for(var o in e)i.d(t,o,function(n){return e[n]}.bind(null,o));return t},i.n=function(e){var n=e&&e.__esModule?function(){return e.default}:function(){return e};return i.d(n,"a",n),n},i.o=function(e,n){return Object.prototype.hasOwnProperty.call(e,n)},i.p="",i.w={},i(i.s=0)}([function(e,n,t){t.e(1).then(t.bind(null,1)).then(e=>{const{RChess:n}=e,t=n.new();console.log("instantiate RChess in worker"),self.addEventListener("message",e=>{const n=e.data,{type:o,payload:r}=n;if("MOVE_PIECE"===o){const{previousPosition:e,move:{source:n,target:o}}=r;!function(e,n){self.postMessage({type:"MOVE_PIECE_RESPONSE",payload:{previousPosition:e,newPosition:n}})}(e,t.move_piece(n+"-"+o).split(" ")[0])}else if("COMPUTE_MOVE"===o){const e=(new Date).getTime(),n=t.compute_move(),o=(new Date).getTime();console.log(`duration: ${(o-e)/1e3}s`),console.log("New IA position: "+n),i=n,self.postMessage({type:"COMPUTE_MOVE_RESPONSE",payload:{newPosition:i}})}var i})})}]);