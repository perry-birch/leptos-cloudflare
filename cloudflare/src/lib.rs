use app::*;
use leptos::*;
use leptos::ssr::render_to_stream;
use worker::*;
use futures::StreamExt;

mod file_details;
use file_details::*;

fn log_request(req: &Request) {
    console_log!(
        "{} - [{}], located at: {:?}, within: {}",
        Date::now().to_string(),
        req.path(),
        req.cf().coordinates().unwrap_or_default(),
        req.cf().region().unwrap_or_else(|| "unknown region".into())
    );
}

#[event(fetch)]
pub async fn main(req: Request, env: worker::Env, _ctx: worker::Context) -> Result<Response> {
    log_request(&req);

    let path = req.path();
    if path == "/" {
        let pkg_path = "/client";
        let head = format!(
            r#"<!DOCTYPE html>
            <html lang="en">
                <head>
                    <meta charset="utf-8"/>
                    <meta name="viewport" content="width=device-width, initial-scale=1"/>
                    <link rel="modulepreload" href="{pkg_path}.js">
                    <link rel="preload" href="{pkg_path}_bg.wasm" as="fetch" type="application/wasm" crossorigin="">
                    <script type="module">import init, {{ hydrate }} from '{pkg_path}.js'; init('{pkg_path}_bg.wasm').then(hydrate);</script>
                </head>
                <body>"#
        );

        let tail = "</body></html>";

        let stream =
            futures::stream::once(async move { head.clone() })
                .chain(render_to_stream( 
                    |cx| view! { cx,  <App /> }.into_view(cx),
                ))
                .chain(futures::stream::once(async { tail.to_string() }))
                .inspect(|html| println!("{html}"))
                .map(|html| Result::Ok(html.into_bytes()));
        let mut response = Response::from_stream(stream)?;
        response.headers_mut()
            // Set the content type header
            .set("Content-Type", "text/html")?;
        return Ok(response);
    }

    // Look for static assets in the KV store
    let store = env.kv("__STATIC_CONTENT")?;
    let asset_keys = store.list().execute().await?.keys;
    let asset_map: Vec<FileDetails> = asset_keys.iter().map(|key| key.into()).collect();

    let file_path = &path[1..];
    // Try to map requested path to KV static file paths
    let file = match asset_map.iter().find(|file| file.file_path().contains(file_path)) {
        None => return Response::error(format!("[{file_path}] Not Found In Asset Map"), 404),
        Some(file) => file,
    };
    // Load the file from the kv store
    let content = match store.get(&file.key).bytes().await? {
        None => return Response::error(format!("[{file_path}] Not Found In KV Store"), 404),
        Some(content) => content,
    };
    // Build the response with the retrieved content
    let mut response = Response::from_bytes(content)?;
    response.headers_mut()
        // Set the content type header
        .set("Content-Type", file.content_type())?;
    Ok(response)
}
