use crate::mdc_sys::MDCRipple;
use web_sys::Element;
use yew::prelude::*;

pub struct Button {
    node_ref: NodeRef,
    ripple: Option<MDCRipple>,
}

#[derive(PartialEq, Clone, Copy, Debug)]
pub enum Style {
    None,
    Raised,
    Unelevated,
    Outlined,
}
impl Default for Style {
    fn default() -> Style {
        Style::None
    }
}
impl std::fmt::Display for Style {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        use Style::*;
        let result = match self {
            None => "",
            Raised => "mdc-button--raised",
            Unelevated => "mdc-button--unelevated",
            Outlined => "mdc-button--outlined",
        };
        write!(f, "{}", result)
    }
}

#[derive(Properties, Clone, Debug, PartialEq)]
pub struct Props {
    #[prop_or_default]
    pub children: Children,
    #[prop_or_default]
    pub id: String,
    pub text: String,
    #[prop_or_default]
    pub style: Style,
    #[prop_or_default]
    pub trailingicon: bool,
    #[prop_or_else(Callback::noop)]
    pub onclick: Callback<MouseEvent>,
    #[prop_or_default]
    pub classes: String,
    #[cfg(feature = "dialog")]
    #[prop_or_default]
    pub dialog_data: Option<String>,
    #[prop_or_default]
    pub disabled: bool,
}

pub enum Msg {
    Clicked(MouseEvent),
}

impl Component for Button {
    type Properties = Props;
    type Message = Msg;

    fn create(_ctx: &Context<Self>) -> Self {
        Self {
            node_ref: NodeRef::default(),
            ripple: None,
        }
    }

    fn rendered(&mut self, _ctx: &Context<Self>, first_render: bool) {
        if first_render {
            if let Some(ripple) = self.ripple.take() {
                ripple.destroy();
            }
            self.ripple = self.node_ref.cast::<Element>().map(MDCRipple::new);
        }
    }

    fn changed(&mut self, _ctx: &Context<Self>) -> bool {
        true
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::Clicked(ev) => {
                ctx.props().onclick.emit(ev);
            }
        }
        false
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        #[cfg(feature = "dialog")]
        let props = ctx.props();
        let props = if props.dialog_data.is_some() && !props.classes.contains("mdc-dialog__button")
        {
            Props {
                classes: props.classes.clone() + " mdc-dialog__button",
                ..(props.clone())
            }
        } else {
            props.clone()
        };

        let inner = if props.trailingicon {
            html! { <>
                <span class="mdc-button__label">{ &props.text }</span>
                { props.children.clone() }
            </> }
        } else {
            html! { <>
                { props.children.clone() }
                <span class="mdc-button__label">{ &props.text }</span>
            </> }
        };
        let classes = format!("mdc-button {} {}", props.style, props.classes);
        if let Some(action) = props.dialog_data.clone() {
            html! {
                <button class={classes}
                        id={props.id.clone()}
                        ref={self.node_ref.clone()}
                        disabled={props.disabled}
                        data-mdc-dialog-action={action}>
                    <div class="mdc-button__ripple"></div>
                    { inner }
                </button>
            }
        } else {
            let onclick = ctx.link().callback(Msg::Clicked);
            html! {
                <button class={classes}
                        id={props.id.clone()}
                        ref={self.node_ref.clone()}
                        disabled={props.disabled}
                        onclick={onclick}>
                    <div class="mdc-button__ripple"></div>
                    { inner }
                </button>
            }
        }
    }

    fn destroy(&mut self, _ctx: &Context<Self>) {
        if let Some(ripple) = &self.ripple {
            ripple.destroy();
        }
    }
}
