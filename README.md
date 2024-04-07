# Libum project

A simple snake game built with rust and wasm-pack.

<br />

## How to use

First, you'd need to clone this project and install the dependencies. For rust, as long as you have cargo and the rust extension you should be fine. For the web, you need a package manager like npm, pnpm or bun to run `[package-manager] install`

<br />

With all of that out of the way, you need to compile the rust code to wasm with [wasm-pack](https://rustwasm.github.io/wasm-pack/installer), to do so you need to run the following in the root of the project 

```bash
wasm-pack build --target web
```

<br />

the folder is already linked in the package.json of the web application, however you can do a double check and see if the names match. Now all that you have to do is run the web application

```bash
cd www && [package-manager] run dev
```
