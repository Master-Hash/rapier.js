<p align="center">
  <img src="https://www.rapier.rs/img/rapier_logo_color_textpath_dark.svg" alt="crates.io">
</p>
<p align="center">
    <a href="https://discord.gg/vt9DJSW">
        <img src="https://img.shields.io/discord/507548572338880513.svg?logo=discord&colorB=7289DA">
    </a>
    <a href="https://travis-ci.org/dimforge/rapier">
        <img src="https://travis-ci.org/dimforge/rapier.svg?branch=master" alt="Build status">
    </a>
    <a href="https://opensource.org/licenses/Apache-2.0">
        <img src="https://img.shields.io/badge/License-Apache%202.0-blue.svg">
    </a>
</p>
<p align = "center">
    <strong>
        <a href="https://rapier.rs">Website</a> | <a href="https://rapier.rs/docs/">Documentation</a> |
        <a href="https://github.com/dimforge/rapier.js/tree/master/testbed2d/src/demos">2D examples (sources)</a> | 
        <a href="https://github.com/dimforge/rapier.js/tree/master/testbed3d/src/demos">3D examples (sources)</a>
</p>

---

<p align = "center">
<b>2D and 3D physics engines</b>
<i>for the JavaScript programming language (official bindings).</i>
</p>

---

在官方 binding 基础上，调整如下：

1. 砍掉 control 模块；
2. wasm-bindgen target web 而不是 bundler，兼容 vite；
3. 启用 -Oz 等编译参数，减少体积；
4. 使用最新的 rapier rust 依赖。

编译参数：

```bash
cd rapier2d
cargo build --release --verbose --target wasm32-unknown-unknown
wasm-bindgen ../target/wasm32-unknown-unknown/release/rapier_wasm2d.wasm --out-dir ./pkg --typescript --target web
wasm-opt ./pkg/rapier_wasm2d_bg.wasm -o ./pkg/rapier_wasm2d_bg.wasm.opt -Oz --dce
./build_typescript.sh
pnpm tsc
```
