# <a href="https://AndrejOrsula.github.io"><img src="./assets/icons/favicon.svg" width="24" height="24"> AndrejOrsula.github.io</a>

<p>
  <a href="https://github.com/AndrejOrsula/AndrejOrsula.github.io/actions/workflows/ci.yml"> <img alt="CI/CD" src="https://github.com/AndrejOrsula/AndrejOrsula.github.io/actions/workflows/ci.yml/badge.svg"></a>
</p>

My portfolio website written in Rust on top of [egui](https://github.com/emilk/egui).

## Instructions

### <a href="#-native-app"><img src="https://rustacean.net/assets/rustacean-flat-noshadow.svg" width="16" height="16"></a> Native App

> \[!NOTE\]
> [Rust](https://www.rust-lang.org) and [Cargo](https://doc.rust-lang.org/stable/cargo) are required for compilation of
> this app. Install them through your package manager or via <https://rustup.rs>.

Compile and run the app locally with Cargo.

```bash
cargo run
```

### <a href="#-web-app"><img src="https://www.svgrepo.com/show/374180/wasm.svg" width="16" height="16"></a> Web App

> \[!NOTE\]
> [WebAssembly](https://www.rust-lang.org/what/wasm) target is required for the compilation of this app for the web.
> Furthermore, [Trunk](https://trunkrs.dev) greatly simplifies the process of building and serving the app.
>
> ```bash
> rustup target add wasm32-unknown-unknown
> cargo install --locked trunk
> ```

You can serve the app locally with Trunk and view it in your browser at <http://localhost:8080>. You can also specify
the `--address` to view the app on other devices in your local network.

```bash
trunk serve --open
```

> \[!TIP\]
> The deployment of the app to [GitHub Pages](https://pages.github.com) is automated through
> the [`ci.yml`](.github/workflows/ci.yml) workflow on every push to the `main` branch.

## License

This project is dual-licensed under either the [MIT](LICENSE-MIT) or [Apache 2.0](LICENSE-APACHE) licenses.
