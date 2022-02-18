use crate::mdc_sys::MDCRipple;
use web_sys::Element;
use yew::prelude::*;

pub struct FAB {
    node_ref: NodeRef,
    ripple: Option<MDCRipple>,
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

    fn create(_ctx: &Context<Self>) -> Self {
        Self {
            node_ref: NodeRef::default(),
            ripple: None,
        }
    }

    fn changed(&mut self, _ctx: &Context<Self>) -> bool {
        true
    }

    fn rendered(&mut self, _ctx: &Context<Self>, first_render: bool) {
        if first_render {
            if let Some(ripple) = self.ripple.take() {
                ripple.destroy();
            }
            self.ripple = self.node_ref.cast::<Element>().map(MDCRipple::new);
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::Clicked(ev) => {
                ctx.props().onclick.emit(ev);
            }
        }
        false
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let ripple = html! { <div class="mdc-fab__ripple"></div> };
        let mini = if ctx.props().mini {
            " mdc-fab--mini"
        } else {
            ""
        };
        let exited = if ctx.props().exited {
            " mdc-fab--exited"
        } else {
            ""
        };
        let (label, extended) = if let Some(text) = &ctx.props().text {
            (
                html! { <span class="mdc-fab__label">{ text }</span> },
                " mdc-fab--extended",
            )
        } else {
            (html! {}, "")
        };
        let classes = format!("mdc-fab{}{}{}", mini, extended, exited);
        let onclick = ctx.link().callback(Msg::Clicked);
        html! {
            <button class={classes} id={ctx.props().id.clone()}
                    ref={self.node_ref.clone()}
                    onclick={onclick}>
                { ripple }
                { ctx.props().children.clone() }
                { label }
            </button>
        }
    }

    fn destroy(&mut self, _ctx: &Context<Self>) {
        if let Some(ripple) = &self.ripple {
            ripple.destroy();
        }
    }
}
