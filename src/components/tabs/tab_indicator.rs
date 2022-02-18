//use crate::mdc_sys::MDCTabIndicator;
use yew::prelude::*;

pub struct TabIndicator {
    //inner: Option<MDCTabIndicator>,
}

#[derive(Properties, Clone, PartialEq)]
pub struct Props {
    #[prop_or_default]
    pub id: String,

    #[prop_or_default]
    pub active: bool,
    #[prop_or_default]
    pub fading: bool,
}

impl Component for TabIndicator {
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
        let fading = if ctx.props().fading {
            " mdc-tab-indicator--fade"
        } else {
            ""
        };
        let active = if ctx.props().active {
            " mdc-tab-indicator--active"
        } else {
            ""
        };
        let classes = format!("mdc-tab-indicator{}{}", fading, active);
        html! {
            <span class={classes} id={ctx.props().id.clone()}>
                <span class="mdc-tab-indicator__content mdc-tab-indicator__content--underline"></span>
            </span>
        }
    }
}
