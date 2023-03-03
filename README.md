# Example of Leptos Deployed to Cloudflare Workers

# Wrangler

You will need to have wrangler installed to run this example.

Wrangler is a cli tool for interacting with Cloudflare services:

[Install/Update Wrangler](https://developers.cloudflare.com/workers/wrangler/install-and-update/)

# Client

Build the client wasm and js bindings with wasm-pack to produce the 'pkg' directory containing the necessary js and wasm files for running the app.

```sh
cd client
wasm-pack build --target=web
```

# Server

The 'wrangler dev' command runs setup script from wrangler.toml and then launches a dev env.

* This script should work but due to a bug in worker-build (?) an alternative is used, see below *

[Workers-rs Issue 282](https://github.com/cloudflare/workers-rs/issues/282)

```toml
[build]
command = "cargo install -q worker-build --version 0.0.7 && worker-build --release"
```

* Temporary build command *

```toml
[build]
command = "cargo install --git https://github.com/CathalMullan/workers-rs worker-build && worker-build --release"
```

Move into the cloudflare directory and run 'wrangler dev' to launch the worker locally:

```sh
cd cloudflare
wrangler dev
```

If this fails with `[mf:err] Unhandled Promise Rejection: Error: To use the new ReadableStream() constructor, enable the streams_enable_constructors feature flag.` 
then run this instead:
```sh
cd cloudflare
wrangler dev --compatibility-flag streams_enable_constructors
```
