use {
    futures::prelude::*,
    gdua_core::{analyze_disk_usage, FileEntry},
    serde_json::to_string,
    std::{env::args, path::PathBuf, str::FromStr, time::Duration},
    tokio::runtime::Runtime,
    tokio_batch::Chunks,
    web_view::{self, Content, WVResult, WebView},
};

const HTML: &str = include_str!(concat!(env!("OUT_DIR"), "/index.html"));

fn fetch_file_entries<T>(webview: &mut WebView<T>, entries: &[FileEntry]) -> WVResult {
    webview.eval(&format!(
        "window.fetch_file_entries({})",
        to_string(entries).unwrap()
    ))
}

fn main() {
    use futures::future::ok;

    let mut args = args();
    args.next();
    let arg = args.next().expect("Path expected");

    let webview = web_view::builder()
        .title("Graphical Disk Usage Analyzer")
        .content(Content::Html(HTML))
        .size(800, 600)
        .resizable(true)
        .debug(cfg!(debug_assertions))
        .user_data(())
        .invoke_handler(|_webview, _arg| Ok(()))
        .build()
        .unwrap();

    let webview_handle = webview.handle();

    let stream = analyze_disk_usage(PathBuf::from_str(&arg).expect("Invalid path"));
    let chunked_stream = Chunks::new(stream, 10, Duration::from_secs(1));

    let fut = chunked_stream.for_each(move |entry| {
        let _ = webview_handle
            .dispatch(move |mut webview| fetch_file_entries(&mut webview, &entry))
            .ok();
        ok(())
    });

    let mut rt = Runtime::new().unwrap();

    rt.spawn(fut.map_err(|e| eprintln!("{:?}", e)));

    webview.run().unwrap();
    rt.shutdown_now().wait().unwrap();
}
