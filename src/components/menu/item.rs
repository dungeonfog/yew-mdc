use web_sys::Element;
use yew::prelude::*;

use crate::mdc_sys::MDCRipple;

pub struct Item {
    node_ref: NodeRef,
    ripple: Option<MDCRipple>,
}

#[derive(Properties, Clone, PartialEq)]
pub struct Props {
    #[prop_or_default]
    pub id: String,
    #[prop_or_default]
    pub text: String,
    #[prop_or_default]
    pub disabled: bool,
    #[prop_or_else(Callback::noop)]
    pub onclick: Callback<MouseEvent>,
    #[prop_or_default]
    pub children: Children,
}

pub enum Msg {
    Clicked(MouseEvent),
}

impl Component for Item {
    type Message = Msg;
    type Properties = Props;

    fn create(_ctx: &Context<Self>) -> Self {
        Self {
            node_ref: NodeRef::default(),
            ripple: None,
        }
    }

    fn changed(&mut self, _ctx: &Context<Self>) -> bool {
        true
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::Clicked(ev) => {
                if !ctx.props().disabled {
                    ctx.props().onclick.emit(ev);
                }
            }
        }
        false
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let classes = if ctx.props().disabled {
            "mdc-deprecated-list-item mdc-deprecated-list-item--disabled"
        } else {
            "mdc-deprecated-list-item"
        };
        let onclick = ctx.link().callback(Msg::Clicked);
        let text = if ctx.props().text.is_empty() {
            html! {}
        } else {
            html! {
                <span class="mdc-deprecated-list-item__text">{ &ctx.props().text }</span>
            }
        };
        html! {
            <li class={classes}
                role="menuitem"
                id={ctx.props().id.clone()}
                onclick={onclick}
                ref={self.node_ref.clone()}>
                <span class="mdc-deprecated-list-item__ripple"></span>
                { ctx.props().children.clone() }
                { text }
            </li>
        }
    }

    fn rendered(&mut self, _ctx: &Context<Self>, first_render: bool) {
        if first_render {
            if let Some(ripple) = self.ripple.take() {
                ripple.destroy();
            }
            self.ripple = self.node_ref.cast::<Element>().map(MDCRipple::new);
        }
    }

    fn destroy(&mut self, _ctx: &Context<Self>) {
        if let Some(ripple) = &self.ripple {
            ripple.destroy();
        }
    }
}
