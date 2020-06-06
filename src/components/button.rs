use crate::mdc_sys::MDCRipple;
use web_sys::Element;
use yew::prelude::*;

pub struct Button {
    node_ref: NodeRef,
    ripple: Option<MDCRipple>,
    props: Props,
    link: ComponentLink<Self>,
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

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        #[cfg(feature = "dialog")]
        let props = if props.dialog_data.is_some() && !props.classes.contains("mdc-dialog__button")
        {
            Props {
                classes: props.classes + " mdc-dialog__button",
                ..props
            }
        } else {
            props
        };
        Self {
            node_ref: NodeRef::default(),
            ripple: None,
            props,
            link,
        }
    }

    fn rendered(&mut self, first_render: bool) {
        if first_render {
            if let Some(ripple) = self.ripple.take() {
                ripple.destroy();
            }
            self.ripple = self.node_ref.cast::<Element>().map(MDCRipple::new);
        }
    }

    fn change(&mut self, props: Props) -> ShouldRender {
        #[cfg(feature = "dialog")]
        let props = if props.dialog_data.is_some() && !props.classes.contains("mdc-dialog__button")
        {
            Props {
                classes: props.classes + " mdc-dialog__button",
                ..props
            }
        } else {
            props
        };
        if self.props != props {
            self.props = props;
            true
        } else {
            false
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::Clicked(ev) => {
                self.props.onclick.emit(ev);
            }
        }
        false
    }

    fn view(&self) -> Html {
        let inner = if self.props.trailingicon {
            html! { <>
                <span class="mdc-button__label">{ &self.props.text }</span>
                { self.props.children.render() }
            </> }
        } else {
            html! { <>
                { self.props.children.render() }
                <span class="mdc-button__label">{ &self.props.text }</span>
            </> }
        };
        let classes = format!("mdc-button {} {}", self.props.style, self.props.classes);
        if let Some(action) = &self.props.dialog_data {
            html! {
                <button class=classes
                        id=&self.props.id
                        ref=self.node_ref.clone()
                        disabled=self.props.disabled
                        data-mdc-dialog-action=action>
                    <div class="mdc-button__ripple"></div>
                    { inner }
                </button>
            }
        } else {
            let onclick = self.link.callback(Msg::Clicked);
            html! {
                <button class=classes
                        id=&self.props.id
                        ref=self.node_ref.clone()
                        disabled=self.props.disabled
                        onclick=onclick>
                    <div class="mdc-button__ripple"></div>
                    { inner }
                </button>
            }
        }
    }

    fn destroy(&mut self) {
        if let Some(ripple) = &self.ripple {
            ripple.destroy();
        }
    }
}
