use crate::mdc_sys::MDCRipple;
use yew::prelude::*;

pub struct Button {
    id: String,
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

#[derive(Properties, Clone, Debug)]
pub struct Props {
    pub children: Children,
    pub id: Option<String>,
    #[props(required)]
    pub text: String,
    pub style: Style,
    pub trailingicon: bool,
    pub onclick: Option<Callback<ClickEvent>>,
    pub classes: String,
    #[cfg(feature = "dialog")]
    pub dialog_data: Option<String>,
    pub disabled: bool,
}

pub enum Msg {
    Clicked(ClickEvent),
}

impl Component for Button {
    type Properties = Props;
    type Message = Msg;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        let id = props
            .id
            .as_ref()
            .map(|id| id.to_owned())
            .unwrap_or_else(|| format!("button-{}", crate::next_id()));
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
            id,
            ripple: None,
            props,
            link,
        }
    }

    fn mounted(&mut self) -> ShouldRender {
        self.ripple = crate::get_element_by_id(&self.id).map(MDCRipple::new);
        false
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
        if self.props.id != props.id {
            self.id = props
                .id
                .as_ref()
                .map(|id| id.to_owned())
                .unwrap_or_else(|| format!("button-{}", crate::next_id()));
        }
        self.props = props;
        true
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::Clicked(ev) => {
                if let Some(callback) = &self.props.onclick {
                    callback.emit(ev);
                }
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
                        id=self.id
                        disabled=self.props.disabled
                        data-mdc-dialog-action=action>
                    <div class="mdc-button__ripple"></div>
                    { inner }
                </button>
            }
        } else {
            let onclick = self.link.callback(|ev| Msg::Clicked(ev));
            html! {
                <button class=classes
                        id=self.id
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
