use crate::mdc_sys::MDCRipple;
use web_sys::Element;
use yew::prelude::*;

pub struct PrimaryAction {
    node_ref: NodeRef,
    ripple: Option<MDCRipple>,
}

#[derive(Properties, Clone, PartialEq)]
pub struct Props {
    pub children: Children,
    #[prop_or_default]
    pub id: String,
    #[prop_or_else(Callback::noop)]
    pub onclick: Callback<MouseEvent>,
    #[prop_or_else(Callback::noop)]
    pub oncontextclick: Callback<MouseEvent>,
}

pub enum Msg {
    LeftClick(MouseEvent),
    RightClick(MouseEvent),
}

impl Component for PrimaryAction {
    type Message = Msg;
    type Properties = Props;

    fn create(_ctx: &Context<Self>) -> Self {
        Self {
            node_ref: NodeRef::default(),
            ripple: None,
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

    fn changed(&mut self, _ctx: &Context<Self>) -> bool {
        true
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::LeftClick(event) => {
                ctx.props().onclick.emit(event);
            }
            Msg::RightClick(event) => {
                if ctx.props().oncontextclick != Callback::noop() {
                    event.prevent_default();
                }
                ctx.props().oncontextclick.emit(event);
            }
        }
        false
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let emit_click = ctx.link().callback(Msg::LeftClick);
        let emit_contextclick = ctx.link().callback(Msg::RightClick);
        html! {
            <div
                id={ctx.props().id.clone()}
                ref={self.node_ref.clone()}
                class="mdc-card__primary-action"
                tabindex="0"
                onclick={emit_click}
                oncontextmenu={emit_contextclick}>
                { ctx.props().children.clone() }
            </div>
        }
    }

    fn destroy(&mut self, _ctx: &Context<Self>) {
        if let Some(ripple) = &self.ripple {
            ripple.destroy();
        }
    }
}
