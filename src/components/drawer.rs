use yew::prelude::*;

pub mod content;
pub use content::Content;
pub mod header;
pub use header::Header;

pub struct Drawer {
    // inner: Option<MDCDrawer>,
}

#[derive(Properties, Clone, PartialEq)]
pub struct Props {
    #[prop_or_default]
    pub id: String,
    pub children: Children,
}

impl Component for Drawer {
    type Message = ();
    type Properties = Props;

    fn create(_ctx: &Context<Self>) -> Self {
        Self {}
    }

    fn changed(&mut self, _ctx: &Context<Self>) -> bool {
        true
    }

    fn update(&mut self, _ctx: &Context<Self>, _msg: Self::Message) -> bool {
        false
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        html! {
            <aside class="mdc-drawer" id={ctx.props().id.clone()}>
                { ctx.props().children.clone() }
            </aside>
        }
    }
}
