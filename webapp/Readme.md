<div align="center">

  <img src="https://github.com/visioniechor/symcode/raw/master/docs/images/visioniechor-banner.png">
  <h1>SymCode Web App</h1>

  <p>
    <strong>The Symbolic Barcode for Humans and Machines</strong>
  </p>

  <h3>
    <a href="https://www.visioniechor.org/symcode-docs">Story</a>
    <span> | </span>
    <a href="https://symcode.visioniechor.org/">Demo</a>
    <span> | </span>
    <a href="https://github.com/visioniechor/acute32">Usage</a>
  </h3>
  <sub>Built with 🦀 by <a href="//www.visioniechor.org/">The Vision iEchor Research Group</a></sub>
</div>

# Synopsis

Since `symcode` is a pure rust programming library, this crate glues everything together and leverage 
the browser's capability for image and video processing.

If you only want to integrate SymCode into your Javascript project, you can simply use the wasm 
binaries provided at [`acute32`](https://github.com/visioniechor/acute32).

# Build

First install the rust wasm toolchain.

```sh
wasm-pack build
```
