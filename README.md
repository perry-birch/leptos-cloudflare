# Example of Leptos Deployed to Cloudflare Workers

# Wrangler

Wrangler is a cli tool for interacting with Cloudflare services:

[Install/Update Wrangler](https://developers.cloudflare.com/workers/wrangler/install-and-update/)

# Client

Build the client wasm and js bindings with wasm-pack:

```sh
cd client
wasm-pack build --target=web
```

# Server

The 'wrangler dev' command runs setup script from wrangler.toml and then launches a dev env

```toml
[build]
command = "cargo install -q worker-build --version 0.0.7 && worker-build --release"
```

```sh
cd cloudflare
wrangler dev
```
