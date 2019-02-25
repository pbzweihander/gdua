use {
    crate::{service::GduaCoreService, FileEntry},
    std::{
        collections::HashSet,
        path::{Path, PathBuf},
    },
    yew::{html, prelude::*},
};

#[derive(Debug, Clone)]
struct Node {
    path: PathBuf,
    children: Vec<Tree>,
}

impl PartialEq for Node {
    fn eq(&self, other: &Self) -> bool {
        self.path.eq(&other.path)
    }
}

#[derive(Debug, Clone)]
struct Leaf {
    path: PathBuf,
    size: u64,
}

impl PartialEq for Leaf {
    fn eq(&self, other: &Self) -> bool {
        self.path.eq(&other.path)
    }
}

#[derive(Debug, Clone)]
enum Tree {
    Node(Node),
    Leaf(Leaf),
}

impl PartialEq for Tree {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Tree::Node(ref n1), Tree::Node(ref n2)) => n1.eq(n2),
            (Tree::Leaf(ref l1), Tree::Leaf(ref l2)) => l1.eq(l2),
            _ => false,
        }
    }
}

pub struct TreeView {
    tree: Vec<Tree>,
    opened_entries: HashSet<PathBuf>,
    entries: HashSet<PathBuf>,
    _service: GduaCoreService,
    fetch_to_chart: Option<Callback<(Vec<u64>, Vec<String>)>>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum TreeViewMsg {
    Nothing,
    ToggleOpened(PathBuf),
    AddFileEntry(FileEntry),
}

#[derive(Clone, PartialEq, Default)]
pub struct TreeViewProps {
    pub fetch_to_chart: Option<Callback<(Vec<u64>, Vec<String>)>>,
}

impl TreeView {
    fn render_li(inner: Html<Self>, nested: usize, onclick_msg: TreeViewMsg) -> Html<Self> {
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

    fn render_node(&self, node: &Node, depth: usize) -> Html<Self> {
        let opened = self.opened_entries.contains(&node.path);

        let inner = html! {
            <>
                <i
                    class=if opened {
                        "fas fa-chevron-down mr-1"
                    } else {
                        "fas fa-chevron-right mr-1"
                    },
                />
                { node.path.file_name().unwrap_or_default().to_string_lossy() }
            </>
        };
        let msg = TreeViewMsg::ToggleOpened(node.path.clone());

        html! {
            <>
                { Self::render_li(inner, depth, msg) }
                {
                    if opened {
                        self.render_list(&node.children, depth + 1)
                    } else {
                        html! { <></> }
                    }
                }
            </>
        }
    }

    fn render_leaf(leaf: &Leaf, depth: usize) -> Html<Self> {
        Self::render_li(
            html! {
                <>
                    { leaf.path.file_name().unwrap_or_default().to_string_lossy() }
                    <span class="badge badge-pill badge-secondary ml-2",>
                        { leaf.size }
                    </span>
                </>
            },
            depth,
            TreeViewMsg::Nothing,
        )
    }

    fn render_tree(&self, tree: &Tree, depth: usize) -> Html<Self> {
        match *tree {
            Tree::Leaf(ref leaf) => Self::render_leaf(leaf, depth),
            Tree::Node(ref node) => self.render_node(node, depth),
        }
    }

    fn render_list(&self, tree: &[Tree], depth: usize) -> Html<Self> {
        html! {
            // <div class=if depth == 0 { "list-group list-group-root" } else { "list-group" },>
            <>
                { for tree.iter().map(|d| self.render_tree(d, depth))}
            </>
            // </div>
        }
    }
}

impl Component for TreeView {
    type Message = TreeViewMsg;
    type Properties = TreeViewProps;

    fn create(props: TreeViewProps, mut link: ComponentLink<Self>) -> Self {
        TreeView {
            tree: vec![],
            opened_entries: HashSet::new(),
            entries: HashSet::new(),
            _service: GduaCoreService::new(link.send_back(TreeViewMsg::AddFileEntry)),
            fetch_to_chart: props.fetch_to_chart,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            TreeViewMsg::Nothing => false,
            TreeViewMsg::ToggleOpened(path) => {
                if let Some(ref callback) = self.fetch_to_chart {
                    callback.emit((
                        vec![10, 20, 40],
                        vec!["foo".to_string(), "bar".to_string(), "baz".to_string()],
                    ));
                }

                if self.opened_entries.contains(&path) {
                    self.opened_entries.remove(&path)
                } else {
                    self.opened_entries.insert(path)
                }
            }
            TreeViewMsg::AddFileEntry(entry) => {
                if !self.entries.contains(&entry.path) {
                    insert_to_tree(&mut self.tree, &entry);
                    self.entries.insert(entry.path);

                    true
                } else {
                    false
                }
            }
        }
    }

    fn change(&mut self, props: TreeViewProps) -> ShouldRender {
        self.fetch_to_chart = props.fetch_to_chart;
        false
    }
}

impl Renderable<TreeView> for TreeView {
    fn view(&self) -> Html<Self> {
        let mut tree = &self.tree;

        while {
            if tree.len() == 1 {
                let node = &tree[0];

                match node {
                    Tree::Leaf(_) => false,
                    Tree::Node(ref n) => {
                        tree = &n.children;
                        true
                    }
                }
            } else {
                false
            }
        } {}

        html! {
            <div class="list-group",>
                { self.render_list(tree, 0) }
            </div>
        }
    }
}

fn merge_tree(tree: &mut Vec<Tree>, mut ancestors: Vec<PathBuf>, leaf: Leaf) {
    if let Some(outermost) = ancestors.pop() {
        for node in tree.iter_mut() {
            if let Tree::Node(ref mut node) = node {
                if node.path == outermost {
                    merge_tree(&mut node.children, ancestors, leaf);
                    return;
                }
            }
        }

        let new_tree = ancestors.into_iter().fold(Tree::Leaf(leaf), |acc, path| {
            Tree::Node(Node {
                path: path,
                children: vec![acc],
            })
        });

        tree.push(Tree::Node(Node {
            path: outermost,
            children: vec![new_tree],
        }));
    } else {
        tree.push(Tree::Leaf(leaf));
    }
}

fn insert_to_tree(tree: &mut Vec<Tree>, entry: &FileEntry) {
    let mut ancestors = entry.path.ancestors();
    ancestors.next();
    let ancestors: Vec<_> = ancestors
        .map(Path::to_path_buf)
        .filter(|p| p.file_name().map(|s| !s.is_empty()).unwrap_or_default())
        .collect();

    let leaf = Leaf {
        path: entry.path.clone(),
        size: entry.size,
    };

    merge_tree(tree, ancestors, leaf);
}
