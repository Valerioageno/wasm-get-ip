<div align="center">
  <h1><code>wasm-get-ip</code></h1>
  <h3>
  <strong>An IP address fetcher written in <a href="https://www.rust-lang.org/">Rust</a> and built with <a href="https://github.com/rustwasm/wasm-pack">wasm-pack</a></strong>
  </h3>
</div>

<br>

## Core concept

This WASM project aim to create a lightweight ip address client-side fetcher. 

Using the official npm.js <a href="https://www.npmjs.com/package/wasm-get-ip">package</a> the developer can inject in all the "ipAddress" class elements the client IP configuring his own custom API endpoint or using the default <a href="https://www.ipify.org/">ipify</a> endpoint.

Project usage:

```javascript
import * as getIp from 'wasm-get-ip';

getIp.fetch()
```

```html
<p class="ipAddress"></p><!-- Or other markup tags that not require value attribute -->
<input class="ipAddress"/>
<textarea class="ipAddress"></textarea>
```

## 🚴 How to start developing

1. ```git clone https://github.com/Valerioageno/wasm-get-ip.git```
2. ```wasm-pack build```
3. enjoy!
4. optional ```npm init wasm-app www```

## 🚀 Deploy

For deploying WebAssembly in production check the instructions at the following link: <a href="https://rustwasm.github.io/docs/book/reference/deploying-to-production.html">https://rustwasm.github.io/docs/book/reference/deploying-to-production.html</a>
## 📝 License

This project is licensed under the MIT License - see the LICENSE_MIT || LICENSE_APACHE file for more information.

