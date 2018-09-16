# wasm-nanoid

Tutorial for your first wasm npm package (for browser).

```sh
docker run -v "$PWD":/usr/src/myapp -w /usr/src/myapp --rm --interactive --tty stereobooster/rust-wasm
USER=stereobooster cargo generate --git https://github.com/rustwasm/wasm-pack-template
cd wasm-nanoid
```

Use your github handle instead of mine (`USER=stereobooster`).

Edit `Cargo.toml`:

```toml
description = "nanoid implemented in wasm"
repository = "https://github.com/stereobooster/wasm-nanoid"
license = "MIT"
```

Edit `README.md`. Run:

```sh
wasm-pack init
```

Add to `.gitignore`:

```
*.log
pkg/*
```

Commit (you may want to do this in different terminal, because git inside Docker is not configured).

```sh
git add .
git commit -m "initial commit"
```

## Implement

I want to build simple thing, so I will reuse existing Nano ID package (they call it crate in Rust).

Add dependency to Cargo.toml (this is like a `package.json` for npm):

```toml
js-sys = "0.2.6"
nanoid = "0.2.0"
```

Edit `src/lib.rs`:

```rs
extern crate cfg_if;
extern crate wasm_bindgen;
extern crate js_sys;
// import nanoid module
extern crate nanoid;

// for [wasm_bindgen] instruction
use wasm_bindgen::prelude::*;

// the function itself
#[wasm_bindgen]
pub fn simpleNanoid() -> js_sys::JsString {
    // generate nanoid and convert value (str) to JsString
    js_sys::JsString::from(nanoid::simple())
}
```

To build run in docker shell:

```sh
wasm-pack init
```

## Setup testing environment

run in OS shell

```
npm init wasm-app example
```

create `package.json` in the root:

```json
{
  "private": true,
  "workspaces": ["*"]
}
```

Edit `.gitignore`:

```
node_modules
```

Edit `example/package.json`:

```js
"devDependencies": {
  "wasm-nanoid": "^0.1.0",
  ...
}
```

Now run example:

```sh
cd example
yarn
yarn start
```

## Publish

If you want to publish run in OS shell, where your npm credentials are configured:

```sh
cd pkg
npm publish
```
