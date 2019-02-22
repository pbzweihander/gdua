use {
    std::fmt::{Debug, Display},
    yew::{html, prelude::*},
};

#[derive(Debug, Clone)]
pub struct TreeViewNode<T> {
    pub label: T,
    pub children: Vec<TreeViewData<T>>,
    pub opened: bool,
}

impl<T> PartialEq for TreeViewNode<T>
where
    T: PartialEq,
{
    fn eq(&self, other: &Self) -> bool {
        self.label.eq(&other.label)
    }
}

#[derive(Debug, Clone)]
pub enum TreeViewData<T> {
    Node(TreeViewNode<T>),
    Leaf(T),
}

impl<T> PartialEq for TreeViewData<T>
where
    T: PartialEq,
{
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (TreeViewData::Node(ref s), TreeViewData::Node(ref o)) => s.eq(o),
            (TreeViewData::Leaf(ref s), TreeViewData::Leaf(ref o)) => s.eq(o),
            _ => false,
        }
    }
}

#[derive(Debug, Clone)]
pub struct TreeView<T> {
    data: Vec<TreeViewData<T>>,
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct TreeViewProps<T> {
    pub data: Vec<TreeViewData<T>>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum TreeViewMsg<T> {
    Nothing,
    ToggleOpened(T),
}

impl<T> TreeView<T>
where
    T: Debug + Display + Clone + Default + PartialEq + 'static,
{
    fn render_li(inner: Html<Self>, nested: usize, onclick_msg: TreeViewMsg<T>) -> Html<Self> {
        let disabled = onclick_msg == TreeViewMsg::Nothing;

        html! {
            <button
                class="list-group-item list-group-item-action",
                style=format!("padding-left: {}px", 30 + nested * 15),
                onclick=|_| onclick_msg.clone(),
                disabled=disabled,
            >
                { inner }
            </button>
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
        let msg = TreeViewMsg::ToggleOpened(node.label.clone());
        let li = Self::render_li(inner, nested, msg);

        if node.opened {
            html! {
                <>
                    { li }
                    { Self::render_list(&node.children, nested + 1) }
                </>
            }
        } else {
            html! {
                <>
                    { li }
                </>
            }
        }
    }

    fn render_leaf(t: &T, nested: usize) -> Html<Self> {
        Self::render_li(html! { <> { t } </> }, nested, TreeViewMsg::Nothing)
    }

    fn render_list(data: &[TreeViewData<T>], nested: usize) -> Html<Self> {
        html! {
            <div class=if nested == 0 { "list-group list-group-root" } else { "list-group" },>
                { for data.iter().map(|d| Self::render_data(d, nested))}
            </div>
        }
    }

    fn render_data(data: &TreeViewData<T>, nested: usize) -> Html<Self> {
        match *data {
            TreeViewData::Leaf(ref d) => Self::render_leaf(d, nested),
            TreeViewData::Node(ref n) => Self::render_node(n, nested),
        }
    }

    fn toggle_data(data: &mut Vec<TreeViewData<T>>, label: &T) -> bool {
        for d in data.iter_mut() {
            match d {
                TreeViewData::Leaf(_) => (),
                TreeViewData::Node(ref mut n) => {
                    if &n.label == label {
                        n.opened = !n.opened;
                        return true;
                    } else if Self::toggle_data(&mut n.children, label) {
                        return true;
                    }
                }
            }
        }
        false
    }
}

impl<T> Component for TreeView<T>
where
    T: Debug + Display + Clone + Default + PartialEq + 'static,
{
    type Message = TreeViewMsg<T>;
    type Properties = TreeViewProps<T>;

    fn create(props: Self::Properties, _: ComponentLink<Self>) -> Self {
        TreeView { data: props.data }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            TreeViewMsg::Nothing => false,
            TreeViewMsg::ToggleOpened(ref label) => Self::toggle_data(&mut self.data, label),
        }
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
