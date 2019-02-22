use {
    futures::prelude::*,
    gdua_core::analyze_disk_usage,
    serde_json::to_string,
    std::{env::args, path::PathBuf, str::FromStr},
    tokio::runtime::Runtime,
    web_view::{self, Content},
};

const HTML: &'static str = include_str!(concat!(env!("OUT_DIR"), "/index.html"));

fn main() {
    use futures::future::ok;

    let mut args = args();
    args.next();
    let arg = args.next().expect("Path expected");

    let webview = web_view::builder()
        .title("Minimal webview example")
        .content(Content::Html(HTML))
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
