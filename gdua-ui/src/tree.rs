use {
    crate::FileEntry,
    std::{
        collections::HashSet,
        path::{Path, PathBuf},
    },
    yew::{html, prelude::*},
};

pub struct TreeView {
    tree: Vec<Tree>,
    opened_entries: HashSet<PathBuf>,
    entries: HashSet<PathBuf>,
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct TreeViewProps {
    pub data: Vec<FileEntry>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum TreeViewMsg {
    Nothing,
    ToggleOpened(PathBuf),
}

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

        let inner = if opened {
            html! {
                <>
                    <i class="fas fa-chevron-down", />
                    { node.path.file_name().unwrap_or_default().to_string_lossy() }
                </>
            }
        } else {
            html! {
                <>
                    <i class="fas fa-chevron-right", />
                    { node.path.file_name().unwrap_or_default().to_string_lossy() }
                </>
            }
        };
        let msg = TreeViewMsg::ToggleOpened(node.path.clone());
        let li = Self::render_li(inner, depth, msg);

        if opened {
            html! {
                <>
                    { li }
                    { self.render_list(&node.children, depth + 1) }
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

    fn render_leaf(leaf: &Leaf, depth: usize) -> Html<Self> {
        Self::render_li(
            html! {
                <>
                    { leaf.path.file_name().unwrap_or_default().to_string_lossy() }
                    { " " }
                    <span class="badge badge-pill badge-secondary",>
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
            <div class=if depth == 0 { "list-group list-group-root" } else { "list-group" },>
                { for tree.iter().map(|d| self.render_tree(d, depth))}
            </div>
        }
    }
}

impl Component for TreeView {
    type Message = TreeViewMsg;
    type Properties = TreeViewProps;

    fn create(props: TreeViewProps, _: ComponentLink<Self>) -> Self {
        TreeView {
            tree: construct_tree(&props.data),
            opened_entries: HashSet::new(),
            entries: props.data.into_iter().map(|entry| entry.path).collect(),
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            TreeViewMsg::Nothing => false,
            TreeViewMsg::ToggleOpened(path) => {
                if self.opened_entries.contains(&path) {
                    self.opened_entries.remove(&path)
                } else {
                    self.opened_entries.insert(path)
                }
            }
        }
    }

    #[allow(clippy::map_entry)]
    fn change(&mut self, props: TreeViewProps) -> ShouldRender {
        let mut changed = false;

        for entry in props.data {
            if !self.entries.contains(&entry.path) {
                insert_to_tree(&mut self.tree, &entry);
                self.entries.insert(entry.path);
                changed = true;
            }
        }

        changed
    }
}

impl Renderable<TreeView> for TreeView {
    fn view(&self) -> Html<Self> {
        html! {
            <>
                { self.render_list(&self.tree, 0) }
            </>
        }
    }
}

fn merge_tree(tree: &mut Vec<Tree>, mut sub_tree: Vec<Node>, leaf: Leaf) {
    if let Some(last) = sub_tree.pop() {
        if let Some((i, _)) = tree.iter().enumerate().find(|(_, n)| {
            if let Tree::Node(n) = n {
                n == &last
            } else {
                false
            }
        }) {
            if let Some(Tree::Node(n)) = tree.get_mut(i) {
                if sub_tree.is_empty() {
                    n.children.push(Tree::Leaf(leaf));
                } else {
                    merge_tree(&mut n.children, sub_tree, leaf);
                }
            }
        } else {
            let new_tree = sub_tree.into_iter().fold(Tree::Leaf(leaf), |acc, node| {
                Tree::Node(Node {
                    path: node.path,
                    children: vec![acc],
                })
            });

            tree.push(Tree::Node(Node {
                path: last.path,
                children: vec![new_tree],
            }));
        }
    }
}

fn insert_to_tree(tree: &mut Vec<Tree>, entry: &FileEntry) {
    let mut ancestors = entry.path.ancestors().map(Path::to_path_buf).filter(|p| {
        p.file_name()
            .map(|s| !s.is_empty())
            .unwrap_or_else(|| false)
    });

    ancestors.next();

    let ancestors = ancestors
        .map(|path| Node {
            path,
            children: vec![],
        })
        .collect();

    let leaf = Leaf {
        path: entry.path.clone(),
        size: entry.size,
    };

    merge_tree(tree, ancestors, leaf);
}

fn construct_tree(entries: &[FileEntry]) -> Vec<Tree> {
    entries.iter().fold(vec![], |mut acc, entry| {
        insert_to_tree(&mut acc, entry);
        acc
    })
}
