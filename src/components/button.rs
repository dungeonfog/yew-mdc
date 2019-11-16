use crate::mdc_sys::MDCRipple;
use web_sys::window;
use yew::prelude::*;

pub struct Button {
    id: String,
    ripple: Option<MDCRipple>,
    props: Props,
}

#[derive(PartialEq, Debug)]
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

#[derive(PartialEq, Properties, Debug)]
pub struct Props {
    pub id: Option<String>,
    #[props(required)]
    pub text: String,
    pub style: Style,
    pub ripple: bool,
    pub onclick: Option<Callback<()>>,
}

pub enum Msg {
    Clicked,
}

impl Component for Button {
    type Properties = Props;
    type Message = Msg;

    fn create(props: Self::Properties, _link: ComponentLink<Self>) -> Self {
        let id = props
            .id
            .as_ref()
            .map(|id| id.to_owned())
            .unwrap_or_else(|| format!("button-{}", crate::next_id()));
        Self {
            id,
            ripple: None,
            props,
        }
    }

    fn mounted(&mut self) -> ShouldRender {
        if self.props.ripple {
            self.ripple = window()
                .and_then(|w| w.document())
                .and_then(|d| d.get_element_by_id(&self.id))
                .map(MDCRipple::new);
        }
        false
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::Clicked => {
                if let Some(callback) = &self.props.onclick {
                    callback.emit(());
                }
            }
        }
        false
    }

    fn view(&self) -> Html<Self> {
        let ripple_surface = if self.props.ripple {
            " mdc-ripple-surface"
        } else {
            ""
        };
        html! {
            <button class=format!("mdc-button{} {}", ripple_surface, self.props.style)
                    id=self.id
                    onclick=|_| Msg::Clicked>
                <span class="mdc-button__label">{ &self.props.text }</span>
            </button>
        }
    }

    fn destroy(&mut self) {
        if let Some(ripple) = &self.ripple {
            ripple.destroy();
        }
    }
}
