use {
    crate::{tree::Tree, PartialEqMutex},
    std::rc::Rc,
    stdweb::{
        web::{document, Element, INonElementParentNode},
        Value,
    },
    yew::{html, prelude::*},
};

#[derive(Debug, Clone, Default, PartialEq)]
pub struct ChartProps {
    pub data: Rc<PartialEqMutex<Vec<Tree>>>,
    pub update: usize,
}

pub struct Chart {
    ctx: Option<Element>,
    chart: Option<Value>,
    data: Rc<PartialEqMutex<Vec<Tree>>>,
}

impl Component for Chart {
    type Message = ();
    type Properties = ChartProps;

    fn create(props: Self::Properties, _: ComponentLink<Self>) -> Self {
        Chart {
            ctx: None,
            chart: None,
            data: props.data,
        }
    }

    fn update(&mut self, _: Self::Message) -> ShouldRender {
        true
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        self.data = props.data;

        if self.ctx.is_none() {
            self.ctx = document().get_element_by_id("chart");
        }

        if let Some(ref ctx) = self.ctx {
            if self.chart.is_none() {
                self.chart = Some(js! {
                    var chart = Sunburst()(@{ctx});

                    window.onresize = function() {
                        chart
                            .width(document.getElementById("chart").offsetWidth)
                            .height(document.getElementById("chart").offsetWidth);
                    };
                    window.onresize();

                    return chart;
                });
            }
        }

        let data = self.data.0.lock().unwrap();
        js! {
            @{&self.chart}.data({ name: "main", children: @{&*data} });
        };

        true
    }
}

impl Renderable<Chart> for Chart {
    fn view(&self) -> Html<Self> {
        html! {
            <div id="chart", class="w-100",></div>
        }
    }
}
