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
                label: "a".to_string(),
                children: vec![
                    TreeViewData::Leaf("b".to_string()),
                    TreeViewData::Leaf("c".to_string()),
                    TreeViewData::Node(TreeViewNode {
                        label: "aa".to_string(),
                        children: vec![
                            TreeViewData::Leaf("bb".to_string()),
                            TreeViewData::Leaf("cc".to_string()),
                        ],
                        opened: false,
                    }),
                ],
                opened: true,
            }),
            TreeViewData::Leaf("d".to_string()),
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
