use crate::widgets::{self, Reqs, View, Widget, WidgetModel};
use protocol::dashboard;
use yew::html;

pub type PageWidget = WidgetModel<Model>;

pub struct Model {
    page: Option<dashboard::Page>,
}

impl Default for Model {
    fn default() -> Self {
        Self { page: None }
    }
}

#[derive(Default, PartialEq, Clone)]
pub struct Props {
    pub page: Option<dashboard::Page>,
}

impl Widget for Model {
    type Message = ();
    type Properties = Props;

    fn recompose(&mut self, props: &Self::Properties) -> Reqs {
        self.page = props.page.to_owned();
        None
    }

    fn main_view(&self) -> View<Self> {
        if let Some(page) = self.page.as_ref() {
            html! {
                <div class="page",>
                    <div class="header",>
                        <p class="title",>{ &page.title }</p>
                        <div class="separator",></div>
                        <p class="subtitle",>{ &page.subtitle }</p>
                    </div>
                    <div class="body",>
                        <widgets::Layout: layout=Some(page.body.clone()), />
                    </div>
                </div>
            }
        } else {
            html! {
                <widgets::Spinner: />
            }
        }
    }
}
