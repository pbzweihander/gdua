mod tree;

use {
    tree::TreeView,
    yew::{html, prelude::*},
};

struct Model {}

impl Component for Model {
    type Message = ();
    type Properties = ();

    fn create(_: Self::Properties, _: ComponentLink<Self>) -> Self {
        Model {}
    }

    fn update(&mut self, _: Self::Message) -> ShouldRender {
        true
    }
}

impl Renderable<Model> for Model {
    fn view(&self) -> Html<Self> {
        use tree::{TreeViewData, TreeViewNode};

        let data = vec![
            TreeViewData::Node(TreeViewNode {
                label: "Lorem ipsum dolor sit amet, consectetur adipiscing elit.".to_string(),
                children: vec![
                    TreeViewData::Leaf(
                        "Phasellus at turpis pharetra, mattis sem et, tincidunt neque.".to_string(),
                    ),
                    TreeViewData::Leaf("Etiam ac augue ut risus euismod elementum.".to_string()),
                    TreeViewData::Node(TreeViewNode {
                        label: "Quisque mattis massa et lorem condimentum rutrum.".to_string(),
                        children: vec![
                            TreeViewData::Leaf(
                                "Nam a massa aliquam, efficitur orci porttitor, facilisis urna."
                                    .to_string(),
                            ),
                            TreeViewData::Leaf(
                                "Cras vitae turpis id magna facilisis lobortis.".to_string(),
                            ),
                        ],
                        opened: false,
                    }),
                ],
                opened: true,
            }),
            TreeViewData::Leaf(
                "Suspendisse a massa in lorem malesuada egestas eu id enim.".to_string(),
            ),
        ];

        html! {
            <div style="padding: 1rem;",>
                <TreeView<String>: data=data,/>
            </div>
        }
    }
}

fn main() {
    yew::start_app::<Model>();
}
