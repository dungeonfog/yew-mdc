use crate::mdc_sys::MDCRipple;
use web_sys::Element;
use yew::prelude::*;

pub struct FAB {
    node_ref: NodeRef,
    ripple: Option<MDCRipple>,
    props: Props,
    link: ComponentLink<Self>,
}

#[derive(Properties, Clone, PartialEq)]
pub struct Props {
    pub children: Children,
    #[prop_or_default]
    pub id: String,
    #[prop_or_default]
    pub text: Option<String>,
    #[prop_or_default]
    pub mini: bool,
    #[prop_or_default]
    pub exited: bool,
    #[prop_or_else(Callback::noop)]
    pub onclick: Callback<MouseEvent>,
}

pub enum Msg {
    Clicked(MouseEvent),
}

impl Component for FAB {
    type Message = Msg;
    type Properties = Props;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self {
            node_ref: NodeRef::default(),
            ripple: None,
            props,
            link,
        }
    }

    fn change(&mut self, props: Props) -> ShouldRender {
        if self.props != props {
            self.props = props;
            true
        } else {
            false
        }
    }

    fn rendered(&mut self, first_render: bool) {
        if first_render {
            if let Some(ripple) = self.ripple.take() {
                ripple.destroy();
            }
            self.ripple = self.node_ref.cast::<Element>().map(MDCRipple::new);
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::Clicked(ev) => {
                self.props.onclick.emit(ev);
            }
        }
        false
    }

    fn view(&self) -> Html {
        let ripple = html! { <div class="mdc-fab__ripple"></div> };
        let mini = if self.props.mini {
            " mdc-fab--mini"
        } else {
            ""
        };
        let exited = if self.props.exited {
            " mdc-fab--exited"
        } else {
            ""
        };
        let (label, extended) = if let Some(text) = &self.props.text {
            (
                html! { <span class="mdc-fab__label">{ text }</span> },
                " mdc-fab--extended",
            )
        } else {
            (html! {}, "")
        };
        let classes = format!("mdc-fab{}{}{}", mini, extended, exited);
        let onclick = self.link.callback(Msg::Clicked);
        html! {
            <button class=classes id=&self.props.id
                    ref=self.node_ref.clone()
                    onclick=onclick>
                { ripple }
                { self.props.children.clone() }
                { label }
            </button>
        }
    }

    fn destroy(&mut self) {
        if let Some(ripple) = &self.ripple {
            ripple.destroy();
        }
    }
}
