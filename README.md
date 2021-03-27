# projektwahl-wasm-webcomponents

nix shell nixpkgs#cargo

https://rustwasm.github.io/book/
https://rustwasm.github.io/wasm-bindgen/web-sys/index.html
https://rustwasm.github.io/wasm-bindgen/api/js_sys/
https://www.webassemblyman.com/webcomponents.html

https://rustwasm.github.io/book/reference/crates.html
https://rustwasm.github.io/book/reference/tools.html
https://rustwasm.github.io/book/reference/code-size.html
https://rustwasm.github.io/book/reference/add-wasm-support-to-crate.html

cargo install cargo-generate

cargo generate --git https://github.com/rustwasm/wasm-pack-template


wasm-pack build

wasm-pack test --headless --firefox

wasm-pack publish