use yew::prelude::*;

pub mod image;
pub use image::Image;
pub mod supporting;
pub use supporting::Supporting;

pub struct Item;

#[derive(Properties, Clone, PartialEq)]
pub struct Props {
    pub children: Children,
    #[prop_or_default]
    pub id: String,
    #[prop_or_default]
    pub classes: String,
    #[prop_or_else(Callback::noop)]
    pub onclick: Callback<MouseEvent>,
    #[prop_or_else(Callback::noop)]
    pub onmouseenter: Callback<MouseEvent>,
    #[prop_or_else(Callback::noop)]
    pub onmouseleave: Callback<MouseEvent>,
}

pub enum Msg {
    Clicked(MouseEvent),
    MouseEnter(MouseEvent),
    MouseLeave(MouseEvent),
}

impl Component for Item {
    type Message = Msg;
    type Properties = Props;

    fn create(_ctx: &Context<Self>) -> Self {
        Self
    }

    fn changed(&mut self, _ctx: &Context<Self>) -> bool {
        true
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        let (callback, event) = match msg {
            Msg::Clicked(ev) => (&ctx.props().onclick, ev),
            Msg::MouseEnter(ev) => (&ctx.props().onmouseenter, ev),
            Msg::MouseLeave(ev) => (&ctx.props().onmouseleave, ev),
        };
        callback.emit(event);
        false
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        html! {
            <li id={ctx.props().id.clone()}
                class={format!("mdc-image-list__item {}", ctx.props().classes)}
                onclick={ctx.link().callback(Msg::Clicked)}
                onmouseenter={ctx.link().callback(Msg::MouseEnter)}
                onmouseleave={ctx.link().callback(Msg::MouseLeave)}
                >
                { ctx.props().children.clone() }
            </li>
        }
    }
}
