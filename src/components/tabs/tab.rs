//use crate::mdc_sys::MDCTab;
use yew::prelude::*;

use super::TabIndicator;

pub struct Tab {
    props: Props,
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

    fn create(props: Self::Properties, _link: ComponentLink<Self>) -> Self {
        Self { props }
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        if self.props != props {
            self.props = props;
            true
        } else {
            false
        }
    }

    fn update(&mut self, _msg: Self::Message) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        let indicator = html! {
            <TabIndicator
                active=self.props.active
                fading=self.props.fading_indicator
            />
        };
        let (outer_indic, inner_indic) = if self.props.content_only_indicator {
            (html! {}, indicator)
        } else {
            (indicator, html! {})
        };
        let classes = if self.props.active {
            "mdc-tab mdc-tab--active"
        } else {
            "mdc-tab"
        };
        html! {
            <button class=classes id=&self.props.id>
                <span class="mdc-tab__content">
                    { self.props.children.render() }
                    { inner_indic }
                </span>
                { outer_indic }
                <span class="mdc-tab__ripple"></span>
            </button>
        }
    }

    fn destroy(&mut self) {}
}
