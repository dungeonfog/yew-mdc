use yew::prelude::*;

pub struct Supporting;

#[derive(Properties, Clone, PartialEq)]
pub struct Props {
    pub children: Children,
    #[prop_or_default]
    pub id: String,
    #[prop_or_default]
    pub classes: String,
}

impl Component for Supporting {
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
            <div id={ctx.props().id.clone()} class={format!("mdc-image-list__supporting {}", ctx.props().classes)}>
                { ctx.props().children.clone() }
            </div>
        }
    }
}
