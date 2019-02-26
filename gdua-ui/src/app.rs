use {
    crate::{
        chart::Chart,
        tree::{Tree, TreeView},
        PartialEqMutex,
    },
    std::{rc::Rc, sync::Mutex},
    yew::{html, prelude::*},
};

pub struct App {
    data: Rc<PartialEqMutex<Vec<Tree>>>,
    update: usize,
}

pub enum AppMsg {
    Update,
}

impl Component for App {
    type Message = AppMsg;
    type Properties = ();

    fn create(_: Self::Properties, _: ComponentLink<Self>) -> Self {
        App {
            data: Rc::new(PartialEqMutex(Mutex::new(vec![]))),
            update: 0,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            AppMsg::Update => self.update += 1,
        }
        true
    }

    fn change(&mut self, _: Self::Properties) -> ShouldRender {
        false
    }
}

impl Renderable<App> for App {
    fn view(&self) -> Html<Self> {
        html! {
            <div
                class="container
                    p-3
                    vh-md-100
                    d-flex flex-column-reverse flex-md-row
                    align-items-start",
            >
                <div class="w-100 w-md-50 h-100
                    mr-md-3 overflow-auto",
                >
                    <TreeView:
                        data=self.data.clone(),
                        update=|_| AppMsg::Update,
                    />
                </div>
                <div class="w-100 w-md-50 h-100
                    ml-md-3 mb-3 mb-md-0
                    d-flex align-items-stretch",
                >
                    <Chart:
                        data=self.data.clone(),
                        update=self.update,
                    />
                </div>
            </div>
        }
    }
}
