use {crate::FileEntry, stdweb::Value, yew::Callback};

pub struct GduaCoreService {
    handle: Option<Value>,
}

impl GduaCoreService {
    pub fn new(callback: Callback<FileEntry>) -> Self {
        let callback = move |entry| {
            callback.emit(entry);
        };

        let handle = js! {
            var callback = @{callback};
            window.fetch_file_entry = function (file_entry) {
                callback(file_entry);
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
