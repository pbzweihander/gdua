use {
    futures::prelude::*,
    std::path::PathBuf,
    tokio::{self, fs},
};

pub use failure::Error;

enum StreamEither<A, B> {
    A(A),
    B(B),
}

impl<A, B> Stream for StreamEither<A, B>
where
    A: Stream,
    B: Stream<Item = A::Item, Error = A::Error>,
{
    type Item = A::Item;
    type Error = A::Error;

    fn poll(&mut self) -> futures::Poll<Option<A::Item>, A::Error> {
        match *self {
            StreamEither::A(ref mut a) => a.poll(),
            StreamEither::B(ref mut b) => b.poll(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FileEntry {
    pub path: PathBuf,
    pub size: u64,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct DirEntry {
    path: PathBuf,
}

#[derive(Debug, Clone, PartialEq, Eq)]
enum Entry {
    File(FileEntry),
    Dir(DirEntry),
}

fn read_dir_entry(
    dir_entry: tokio_fs::DirEntry,
) -> impl Future<Item = Entry, Error = Error> + Send {
    use futures::future::poll_fn;

    let path = dir_entry.path();

    poll_fn(move || dir_entry.poll_metadata())
        .map_err(Into::into)
        .map(move |metadata| {
            if metadata.is_dir() {
                Entry::Dir(DirEntry { path })
            } else {
                Entry::File(FileEntry {
                    path,
                    size: metadata.len(),
                })
            }
        })
}

fn walk_entry(entry: Entry) -> impl Stream<Item = FileEntry, Error = Error> + Send {
    use futures::stream::once;

    match entry {
        Entry::Dir(dir_entry) => {
            let a = walk_path(dir_entry.path);
            StreamEither::A(a)
        }
        Entry::File(file_entry) => {
            let b = once::<_, Error>(Ok(file_entry));
            StreamEither::B(b)
        }
    }
}

fn walk_path(path: PathBuf) -> impl Stream<Item = FileEntry, Error = Error> + Send {
    use futures::{future::ok, sync::mpsc};

    fs::read_dir(path)
        .flatten_stream()
        .map_err(Into::into)
        .and_then(read_dir_entry)
        .and_then(|entry| {
            let (sender, receiver) = mpsc::unbounded();

            let handle = tokio::spawn(
                walk_entry(entry)
                    .forward(sender)
                    .map(|_| ())
                    .map_err(|e| panic!("{}", e)),
            );

            handle
                .into_future()
                .map_err(|_| unreachable!())
                .and_then(|_| ok(receiver.map_err(|_| unreachable!())))
        })
        .flatten()
}

pub fn analyze_disk_usage(path: PathBuf) -> impl Stream<Item = FileEntry, Error = Error> {
    walk_path(path)
}
