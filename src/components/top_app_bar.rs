use yew::prelude::*;

pub mod section;
pub use section::Section;

pub struct TopAppBar;

#[derive(Properties, Clone, PartialEq)]
pub struct Props {
    pub children: Children,
    #[prop_or_default]
    pub id: String,
    #[prop_or_default]
    pub manualrows: bool,
    #[prop_or_default]
    pub classes: String,
}

impl Component for TopAppBar {
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
        let classes = format!("mdc-top-app-bar {}", ctx.props().classes);
        if ctx.props().manualrows {
            html! {
                <header class={classes} id={ctx.props().id.clone()}>
                    { ctx.props().children.clone() }
                </header>
            }
        } else {
            html! {
                <header class={classes} id={ctx.props().id.clone()}>
                    <div class="mdc-top-app-bar__row">
                        { ctx.props().children.clone() }
                    </div>
                </header>
            }
        }
    }

    // TODO: Wrap related JavaScript
    // TODO: Do we need it?
    //fn destroy(&mut self) {}
}
