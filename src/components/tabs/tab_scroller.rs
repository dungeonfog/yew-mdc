//use crate::mdc_sys::MDCTabScroller;
use yew::prelude::*;

pub struct TabScroller {
    //inner: Option<MDCTabScroller>,
}

#[derive(Properties, Clone, PartialEq)]
pub struct Props {
    pub children: Children,

    #[prop_or_default]
    pub id: String,
}

impl Component for TabScroller {
    type Message = ();
    type Properties = Props;

    fn create(_ctx: &Context<Self>) -> Self {
        Self {}
    }

    fn changed(&mut self, _ctx: &Context<Self>) -> bool {
        true
    }

    fn update(&mut self, _ctx: &Context<Self>, _msg: Self::Message) -> bool {
        false
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        html! {
            <div class="mdc-tab-scroller" id={ctx.props().id.clone()}>
                <div class="mdc-tab-scroller__scroll-area">
                    <div class="mdc-tab-scroller__scroll-content">
                        { ctx.props().children.clone() }
                    </div>
                </div>
            </div>
        }
    }
}
