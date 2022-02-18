use boolinator::Boolinator;
use yew::prelude::*;

pub struct Item;

#[derive(Properties, Clone, PartialEq)]
pub struct Props {
    #[prop_or_default]
    pub id: String,
    #[prop_or_default]
    pub text: String,
    #[prop_or_default]
    pub value: String,
    #[prop_or_default]
    pub disabled: bool,
    #[prop_or_default]
    pub children: Children,
    #[prop_or_else(Callback::noop)]
    pub onclick: Callback<MouseEvent>,
}

impl Component for Item {
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
        let classes = classes![
            "mdc-deprecated-list-item",
            ctx.props()
                .disabled
                .as_some("mdc-deprecated-list-item--disabled")
        ];
        let text = if ctx.props().text.is_empty() {
            html! {}
        } else {
            html! {
                <span class="mdc-deprecated-list-item__text">{ &ctx.props().text }</span>
            }
        };
        html! {
            <li class={classes}
                role="option"
                id={ctx.props().id.clone()}
                data-value={ctx.props().value.clone()}>

                { ctx.props().children.clone() }
                <span class="mdc-deprecated-list-item__ripple"></span>
                { text }
            </li>
        }
    }
}
