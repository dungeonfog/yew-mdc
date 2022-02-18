use yew::prelude::*;

pub struct Item;

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
        Self
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
            "mdc-list-item mdc-list-item--disabled"
        } else {
            "mdc-list-item"
        };
        let onclick = ctx.link().callback(Msg::Clicked);
        let text = if ctx.props().text.is_empty() {
            html! {}
        } else {
            html! {
                <span class="mdc-list-item__text">{ &ctx.props().text }</span>
            }
        };
        html! {
            <li class={classes} role="menuitem" id={ctx.props().id.clone()} onclick={onclick}>
                { ctx.props().children.clone() }
                { text }
            </li>
        }
    }
}
