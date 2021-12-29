//use crate::mdc_sys::MDCTabScroller;
use yew::prelude::*;

pub struct TabScroller {
    props: Props,
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
        html! {
            <div class="mdc-tab-scroller" id=self.props.id.clone()>
                <div class="mdc-tab-scroller__scroll-area">
                    <div class="mdc-tab-scroller__scroll-content">
                        { self.props.children.clone() }
                    </div>
                </div>
            </div>
        }
    }

    fn destroy(&mut self) {}
}
