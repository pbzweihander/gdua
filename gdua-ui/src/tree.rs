use {
    std::fmt::{Debug, Display},
    yew::{html, prelude::*},
};

#[derive(Debug, Clone, PartialEq)]
pub struct TreeViewNode<T> {
    pub label: T,
    pub children: Vec<TreeViewData<T>>,
    pub opened: bool,
}

#[derive(Debug, Clone, PartialEq)]
pub enum TreeViewData<T> {
    Node(TreeViewNode<T>),
    Leaf(T),
}

#[derive(Debug, Clone)]
pub struct TreeView<T> {
    data: Vec<TreeViewData<T>>,
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct TreeViewProps<T> {
    pub data: Vec<TreeViewData<T>>,
}

impl<T> TreeView<T>
where
    T: Debug + Display + Clone + Default + PartialEq + 'static,
{
    fn render_li(inner: Html<Self>, nested: usize) -> Html<Self> {
        html! {
            <li
                class="list-group-item",
                style=format!("padding-left: {}px", 30 + nested * 15),
            >
                { inner }
            </li>
        }
    }

    fn render_node(node: &TreeViewNode<T>, nested: usize) -> Html<Self> {
        let inner = if node.opened {
            html! {
                <>
                    <i class="fas fa-chevron-down", />
                    { &node.label }
                </>
            }
        } else {
            html! {
                <>
                    <i class="fas fa-chevron-right", />
                    { &node.label }
                </>
            }
        };

        if node.opened {
            html! {
                <>
                    { Self::render_li(inner, nested) }
                    { Self::render_list(&node.children, nested + 1) }
                </>
            }
        } else {
            html! {
                <>
                    { Self::render_li(inner, nested) }
                </>
            }
        }
    }

    fn render_leaf(t: &T, nested: usize) -> Html<Self> {
        Self::render_li(html! { <> { t } </> }, nested)
    }

    fn render_list(data: &[TreeViewData<T>], nested: usize) -> Html<Self> {
        html! {
            <ul class=if nested == 0 { "list-group list-group-root" } else { "list-group" },>
                { for data.iter().map(|d| Self::render_data(d, nested))}
            </ul>
        }
    }

    fn render_data(data: &TreeViewData<T>, nested: usize) -> Html<Self> {
        match *data {
            TreeViewData::Leaf(ref d) => Self::render_leaf(d, nested),
            TreeViewData::Node(ref n) => Self::render_node(n, nested),
        }
    }
}

impl<T> Component for TreeView<T>
where
    T: Debug + Display + Clone + Default + PartialEq + 'static,
{
    type Message = ();
    type Properties = TreeViewProps<T>;

    fn create(props: Self::Properties, _: ComponentLink<Self>) -> Self {
        TreeView { data: props.data }
    }

    fn update(&mut self, _: Self::Message) -> ShouldRender {
        true
    }
}

impl<T> Renderable<TreeView<T>> for TreeView<T>
where
    T: Debug + Display + Clone + Default + PartialEq + 'static,
{
    fn view(&self) -> Html<Self> {
        html! {
            <div style="width: 50%;",>
                { Self::render_list(&self.data, 0) }
            <div/>
        }
    }
}
