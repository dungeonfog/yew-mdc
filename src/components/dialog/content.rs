use yew::prelude::*;

pub struct Content;

#[derive(Properties, Clone, PartialEq)]
pub struct Props {
    children: Children,
}

impl Component for Content {
    type Message = ();
    type Properties = Props;

    fn create(_ctx: &Context<Self>) -> Self {
        Self
    }
    fn changed(&mut self, _ctx: &Context<Self>) -> bool {
        true
    }
    fn update(&mut self, _ctx: &Context<Self>, _msg: Self::Message) -> bool {
        false
    }
    fn view(&self, ctx: &Context<Self>) -> Html {
        html! {
            <div class="mdc-dialog__content">
                { ctx.props().children.clone() }
            </div>
        }
    }
}
