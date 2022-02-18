use yew::prelude::*;

pub mod primary_action;
pub use primary_action::PrimaryAction;
pub mod media;
pub use media::Media;
pub use media::Style as MediaStyle;

pub struct Card;

#[derive(Properties, Clone, PartialEq)]
pub struct Props {
    pub children: Children,
    #[prop_or_default]
    pub id: String,
    #[prop_or_default]
    pub outlined: bool,
    #[prop_or_default]
    pub classes: String,
    #[prop_or_default]
    pub raw_css: String,
    #[prop_or_else(Callback::noop)]
    pub onclick: Callback<MouseEvent>,
    #[prop_or_else(Callback::noop)]
    pub oncontextclick: Callback<MouseEvent>,
    #[prop_or_else(Callback::noop)]
    pub onhoverenter: Callback<MouseEvent>,
    #[prop_or_else(Callback::noop)]
    pub onhoverleave: Callback<MouseEvent>,
}

pub enum Msg {
    LeftClick(MouseEvent),
    RightClick(MouseEvent),
    HoverEnter(MouseEvent),
    HoverLeave(MouseEvent),
}

impl Component for Card {
    type Message = Msg;
    type Properties = Props;

    fn create(_ctx: &Context<Self>) -> Self {
        Self
    }

    fn changed(&mut self, _ctx: &Context<Self>) -> bool {
        true
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::LeftClick(ev) => ctx.props().onclick.emit(ev),
            Msg::RightClick(ev) => {
                if ctx.props().oncontextclick != Callback::noop() {
                    ev.prevent_default();
                    ctx.props().oncontextclick.emit(ev);
                }
            }
            Msg::HoverEnter(ev) => ctx.props().onhoverenter.emit(ev),
            Msg::HoverLeave(ev) => ctx.props().onhoverleave.emit(ev),
        }
        false
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let outlined = if ctx.props().outlined {
            "mdc-card--outlined"
        } else {
            ""
        };
        let classes = format!("mdc-card {} {}", ctx.props().classes, outlined);
        let emit_click = ctx.link().callback(Msg::LeftClick);
        let emit_contextclick = ctx.link().callback(Msg::RightClick);
        let emit_hoverenter = ctx.link().callback(Msg::HoverEnter);
        let emit_hoverleave = ctx.link().callback(Msg::HoverLeave);
        html! {
            <div class={classes} id={ctx.props().id.clone()}
                 style={ctx.props().raw_css.clone()}
                 onclick={emit_click}
                 oncontextmenu={emit_contextclick}
                 onmouseenter={emit_hoverenter}
                 onmouseleave={emit_hoverleave}>
                { ctx.props().children.clone() }
            </div>
        }
    }
}
