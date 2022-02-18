use yew::prelude::*;

pub struct HelperLine;

#[derive(Properties, Clone, PartialEq)]
pub struct Props {
    pub children: Children,
    #[prop_or_default]
    pub persistent: bool,
    #[prop_or_default]
    pub validation_msg: bool,
}

impl Component for HelperLine {
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
        let persistent = if ctx.props().persistent {
            " mdc-text-field-helper-text--persistent"
        } else {
            ""
        };
        let validation_msg = if ctx.props().validation_msg {
            " mdc-text-field-helper-text--validation-msg"
        } else {
            ""
        };
        let classes = format!("mdc-text-field-helper-text{}{}", persistent, validation_msg);
        html! {
            <div class="mdc-text-field-helper-line">
                <div class={classes} aria-hidden="true">
                    { ctx.props().children.clone() }
                </div>
            </div>
        }
    }
}
