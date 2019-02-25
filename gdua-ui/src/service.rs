use {crate::FileEntry, stdweb::Value, yew::Callback};

pub struct GduaCoreService {
    handle: Option<Value>,
}

impl GduaCoreService {
    pub fn new(callback: Callback<Vec<FileEntry>>) -> Self {
        let callback = move |entries| {
            callback.emit(entries);
        };

        let handle = js! {
            var callback = @{callback};
            window.fetch_file_entries = function (file_entries) {
                callback(file_entries);
            };

            return {
                callback: callback,
            };
        };

        GduaCoreService {
            handle: Some(handle),
        }
    }
}

impl Drop for GduaCoreService {
    fn drop(&mut self) {
        if let Some(handle) = self.handle.take() {
            js! { @(no_return)
                var handle = @{handle};
                handle.callback.drop();
            }
        }
    }
}
