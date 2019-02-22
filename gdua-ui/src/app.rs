use {
    crate::tree::TreeView,
    yew::{html, prelude::*},
};

pub struct App {}

impl Component for App {
    type Message = ();
    type Properties = ();

    fn create(_: Self::Properties, _: ComponentLink<Self>) -> Self {
        App {}
    }

    fn update(&mut self, _: Self::Message) -> ShouldRender {
        true
    }
}

impl Renderable<App> for App {
    fn view(&self) -> Html<Self> {
        html! {
            <div style="padding: 2rem;",>
                <TreeView: />
            </div>
        }
    }
}
