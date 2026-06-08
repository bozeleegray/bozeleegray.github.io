# Polar Landing Page (Rust + WebAssembly)

A landing page for a polar exploration & Arctic science portfolio, written in
[Rust](https://www.rust-lang.org/) with the [Yew](https://yew.rs/) framework,
compiled to WebAssembly, and bundled by [Trunk](https://trunkrs.dev/).

## Local development

```bash
# one-time toolchain setup
rustup target add wasm32-unknown-unknown
cargo install trunk          # or grab a prebuilt binary from the Trunk releases

# live-reloading dev server at http://127.0.0.1:8080
trunk serve

# production build into ./dist
trunk build --release
```

## Project structure

| Path                | Purpose                                            |
| ------------------- | -------------------------------------------------- |
| `src/main.rs`       | Yew application — markup and page content          |
| `index.html`        | Trunk entry point (links the Rust + CSS assets)    |
| `styles/main.css`   | Styling (aurora background, hero, cards)           |
| `Trunk.toml`        | Trunk build/serve configuration                    |

## Deployment to GitHub Pages

Deployment is automated by `.github/workflows/deploy-web.yml`. On every push to
`main` that touches `web/`, the workflow builds the site and publishes it with
GitHub Pages.

**One-time setup:** in the repository settings, go to **Settings → Pages** and
set **Source** to **GitHub Actions**.

Because this is a *project* page, the site is served from a sub-path, so the CI
build uses `--public-url /non-programming/`. The live URL will be:

```
https://bozeleegray.github.io/non-programming/
```

> If you later move this to a dedicated `bozeleegray.github.io` repo (a *user*
> page served from the root), drop the `--public-url` flag in the workflow.
