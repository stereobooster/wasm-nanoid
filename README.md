# wasm-nanoid

Tutorial for your first wasm package.

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

```
git add .
git commit -m "initial commit"
```
