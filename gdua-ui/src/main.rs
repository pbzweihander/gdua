mod tree;

use {
    tree::TreeView,
    yew::{html, prelude::*},
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FileEntry {
    pub path: std::path::PathBuf,
    pub size: u64,
}

struct App {
    data: Vec<FileEntry>,
}

impl Component for App {
    type Message = ();
    type Properties = ();

    fn create(_: Self::Properties, _: ComponentLink<Self>) -> Self {
        let data = vec![];

        App { data }
    }

    fn update(&mut self, _: Self::Message) -> ShouldRender {
        true
    }
}

impl Renderable<App> for App {
    fn view(&self) -> Html<Self> {
        html! {
            <div style="padding: 2rem;",>
                <TreeView: data=self.data.clone(),/>
            </div>
        }
    }
}

fn main() {
    yew::start_app::<App>();
}
