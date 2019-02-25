use {
    serde_derive::Serialize,
    stdweb::{
        js_serializable,
        web::{document, Element, INonElementParentNode},
        Value,
    },
    yew::{html, prelude::*},
};

#[derive(Debug, Clone, Serialize)]
struct ChartDataset {
    data: Vec<u64>,
    #[serde(rename = "backgroundColor")]
    background_color: Vec<String>,
}

#[derive(Debug, Clone, Serialize)]
struct ChartData {
    datasets: Vec<ChartDataset>,
    labels: Vec<String>,
}

impl From<(Vec<u64>, Vec<String>)> for ChartData {
    fn from(d: (Vec<u64>, Vec<String>)) -> Self {
        ChartData {
            datasets: vec![ChartDataset {
                data: d.0,
                background_color: vec!["#ff0000".to_string(); d.1.len()],
            }],
            labels: d.1,
        }
    }
}

js_serializable!(ChartData);

#[derive(Debug, Clone, Default, PartialEq)]
pub struct ChartProps {
    pub data: Vec<u64>,
    pub labels: Vec<String>,
}

pub struct Chart {
    ctx: Option<Element>,
    chart: Option<Value>,
    data: ChartData,
}

impl Component for Chart {
    type Message = ();
    type Properties = ChartProps;

    fn create(props: Self::Properties, _: ComponentLink<Self>) -> Self {
        js! {
            window.onbeforeprint = function () {
                for (var id in Chart.instances) {
                    Chart.instances[id].resize();
                }
            };
        };

        let data = (props.data, props.labels).into();

        Chart {
            ctx: None,
            chart: None,
            data,
        }
    }

    fn update(&mut self, _: Self::Message) -> ShouldRender {
        true
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        self.data = (props.data, props.labels).into();

        if self.ctx.is_none() {
            self.ctx = document().get_element_by_id("chart");
        }

        if let Some(ref ctx) = self.ctx {
            if self.chart.is_none() {
                self.chart = Some(js! {
                    var chart = new Chart(@{ctx}, {
                        type: "doughnut",
                        data: @{&self.data},
                        options: {
                            legend: {
                                display: false,
                            },
                            layout: {
                                padding: 20,
                            },
                            aspectRatio: 1,
                        },
                    });
                    return chart;
                });
            }
        }

        if self.ctx.is_some() && self.chart.is_some() {
            js! {
                @{&self.chart}.data = @{&self.data};
                @{&self.chart}.update();
            }
        }

        true
    }
}

impl Renderable<Chart> for Chart {
    fn view(&self) -> Html<Self> {
        html! {
            <canvas id="chart",></canvas>
        }
    }
}
