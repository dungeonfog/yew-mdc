//use crate::mdc_sys::MDCTabIndicator;
use yew::prelude::*;

pub struct TabIndicator {
    props: Props,
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

    fn create(props: Self::Properties, _link: ComponentLink<Self>) -> Self {
        Self { props }
    }

    fn mounted(&mut self) -> ShouldRender {
        false
    }

    fn update(&mut self, _msg: Self::Message) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        let fading = if self.props.fading {
            " mdc-tab-indicator--fade"
        } else {
            ""
        };
        let active = if self.props.active {
            " mdc-tab-indicator--active"
        } else {
            ""
        };
        let classes = format!("mdc-tab-indicator{}{}", fading, active);
        html! {
            <span class=classes id=&self.props.id>
                <span class="mdc-tab-indicator__content mdc-tab-indicator__content--underline"></span>
            </span>
        }
    }

    fn destroy(&mut self) {}
}
