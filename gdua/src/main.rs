use {
    futures::prelude::*,
    gdua_core::analyze_disk_usage,
    serde_json::to_string,
    std::{env::args, path::PathBuf, str::FromStr},
    tokio::runtime::Runtime,
    web_view::{self, Content},
};

fn main() {
    use futures::future::ok;

    let html = format!(
        r#"
        <!DOCTYPE html>
        <html lang="en">
        <head>
            <meta charset="utf-8" />
            <meta
            name="viewport"
            content="width=device-width, initial-scale=1, shrink-to-fit=no"
            />

            <link
            rel="stylesheet"
            href="https://stackpath.bootstrapcdn.com/bootstrap/4.3.1/css/bootstrap.min.css"
            integrity="sha384-ggOyR0iXCbMQv3Xipma34MD+dH/1fQ784/j6cY/iJTQUOhcWr7x9JvoRxT2MZw1T"
            crossorigin="anonymous"
            />
            <link
            rel="stylesheet"
            href="https://use.fontawesome.com/releases/v5.7.2/css/all.css"
            integrity="sha384-fnmOCqbTlWIlj8LyTjo7mOUStjsKC4pOpQbqyi7RrhN7udi9RwhKkMHpvLbHG9Sr"
            crossorigin="anonymous"
            />
            {style}

            <title>Hello, world!</title>
        </head>

        <body>
            <script
            src="https://code.jquery.com/jquery-3.3.1.slim.min.js"
            integrity="sha384-q8i/X+965DzO0rT7abK41JStQIAqVgRVzpbzo5smXKp4YfRvH+8abtTE1Pi6jizo"
            crossorigin="anonymous"
            ></script>
            <script
            src="https://cdnjs.cloudflare.com/ajax/libs/popper.js/1.14.7/umd/popper.min.js"
            integrity="sha384-UO2eT0CpHqdSJQ6hJty5KVphtPhzWj9WO1clHTMGa3JDZwrnQq4sF86dIHNDz0W1"
            crossorigin="anonymous"
            ></script>
            <script
            src="https://stackpath.bootstrapcdn.com/bootstrap/4.3.1/js/bootstrap.min.js"
            integrity="sha384-JjSmVgyd0p3pXB1rRibZUAYoIIy6OrQ6VrjIEaFf/nJGzIxFDsf4x0xIM+B07jRM"
            crossorigin="anonymous"
            ></script>
            {script}
        </body>
        </html>
    "#,
        style = inline_style(include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/../target/deploy/styles.css"
        ))),
        script = inline_script(include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/../target/deploy/gdua-ui.js"
        ))),
    );

    let mut args = args();
    args.next();
    let arg = args.next().expect("Path expected");

    let webview = web_view::builder()
        .title("Minimal webview example")
        .content(Content::Html(html))
        .size(800, 600)
        .resizable(true)
        .debug(cfg!(debug_assertions))
        .user_data(())
        .invoke_handler(|_webview, _arg| Ok(()))
        .build()
        .unwrap();

    let webview_handle = webview.handle();

    let fut =
        analyze_disk_usage(PathBuf::from_str(&arg).expect("Invalid path")).for_each(move |entry| {
            let _ = webview_handle
                .dispatch(move |webview| {
                    webview.eval(&format!(
                        "window.fetch_file_entry({})",
                        to_string(&entry).unwrap()
                    ))
                })
                .ok();
            ok(())
        });

    let mut rt = Runtime::new().unwrap();

    rt.spawn(fut.map_err(|e| eprintln!("{}", e)));

    webview.run().unwrap();
    rt.shutdown_now().wait().unwrap();
}

fn inline_style(s: &str) -> String {
    format!(r#"<style type="text/css">{}</style>"#, s)
}

fn inline_script(s: &str) -> String {
    format!(r#"<script type="text/javascript">{}</script>"#, s)
}
