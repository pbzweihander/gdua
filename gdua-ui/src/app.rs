use {
    crate::{chart::Chart, tree::TreeView},
    yew::{html, prelude::*},
};

pub struct App {
    chart_data: (Vec<u64>, Vec<String>),
}

pub enum AppMsg {
    FetchToChart((Vec<u64>, Vec<String>)),
}

impl Component for App {
    type Message = AppMsg;
    type Properties = ();

    fn create(_: Self::Properties, _: ComponentLink<Self>) -> Self {
        App {
            chart_data: (vec![], vec![]),
        }
    }

    fn update(&mut self, msg: AppMsg) -> ShouldRender {
        match msg {
            AppMsg::FetchToChart(data) => self.chart_data = data,
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
                    mr-md-3
                    overflow-auto",
                >
                    <TreeView: fetch_to_chart=AppMsg::FetchToChart,/>
                </div>
                <div class="w-100 w-md-50
                    ml-md-3 mb-3 mb-md-0",
                >
                    <Chart:
                        data=self.chart_data.0.clone(),
                        labels=self.chart_data.1.clone(),
                    />
                </div>
            </div>
        }
    }
}
