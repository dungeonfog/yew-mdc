//use crate::mdc_sys::MDCTab;
use yew::prelude::*;

use super::TabIndicator;

pub struct Tab {
    //inner: Option<MDCTab>,
}

#[derive(Properties, Clone, PartialEq)]
pub struct Props {
    pub children: Children,

    #[prop_or_default]
    pub id: String,
    #[prop_or_default]
    pub active: bool,

    /// If set to true, this tab's indicator will only span the tab's content.
    #[prop_or_default]
    pub content_only_indicator: bool,
    #[prop_or_default]
    pub fading_indicator: bool,
}

impl Component for Tab {
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
        let indicator = html! {
            <TabIndicator
                active={ctx.props().active}
                fading={ctx.props().fading_indicator}
            />
        };
        let (outer_indic, inner_indic) = if ctx.props().content_only_indicator {
            (html! {}, indicator)
        } else {
            (indicator, html! {})
        };
        let classes = if ctx.props().active {
            "mdc-tab mdc-tab--active"
        } else {
            "mdc-tab"
        };
        html! {
            <button class={classes} id={ctx.props().id.clone()}>
                <span class="mdc-tab__content">
                    { ctx.props().children.clone() }
                    { inner_indic }
                </span>
                { outer_indic }
                <span class="mdc-tab__ripple"></span>
            </button>
        }
    }
}
