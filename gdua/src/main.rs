use {
    futures::prelude::*,
    gdua_core::{analyze_disk_usage, Error, FileEntry},
    std::{env::args, path::PathBuf, str::FromStr},
    tokio,
};

fn print_entry(entry: &FileEntry) {
    println!("{}\t{}", entry.size, entry.path.display());
}

fn main() {
    use futures::future::ok;

    let mut args = args();
    args.next();
    let arg = args.next().expect("Path expected");

    let fut = analyze_disk_usage(PathBuf::from_str(&arg).expect("Invalid path"))
        .inspect(|entry| {
            print_entry(entry);
        })
        .fold(0, |acc, entry| ok::<_, Error>(acc + entry.size))
        .map(|sum| println!("{}", sum));

    tokio::run(fut.map_err(|e| eprintln!("{}", e)))
}
