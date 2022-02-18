use std::fmt;
use yew::prelude::*;

pub struct Section;

#[derive(PartialEq, Clone, Copy, Debug)]
pub enum Align {
    Start,
    End,
}

impl Default for Align {
    fn default() -> Align {
        Align::Start
    }
}
impl fmt::Display for Align {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let align = match self {
            Align::Start => "start",
            Align::End => "end",
        };
        write!(f, "mdc-top-app-bar__section--align-{}", align)
    }
}

#[derive(Properties, Clone, PartialEq)]
pub struct Props {
    pub children: Children,
    #[prop_or_default]
    pub id: String,
    #[prop_or_default]
    pub align: Align,
}

impl Component for Section {
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
        let classes = format!("mdc-top-app-bar__section {}", ctx.props().align);
        html! {
            <section class={classes} id={ctx.props().id.clone()}>
                { ctx.props().children.clone() }
            </section>
        }
    }
}
