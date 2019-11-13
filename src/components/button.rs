use crate::mdc_sys::MDCRipple;
use web_sys::window;
use yew::prelude::*;

pub struct Button {
    ripple: Option<MDCRipple>,
    props: Props,
}

#[derive(PartialEq)]
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

#[derive(PartialEq, Properties)]
pub struct Props {
    #[props(required)]
    pub id: String,
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
        Self {
            ripple: None,
            props,
        }
    }

    fn mounted(&mut self) -> ShouldRender {
        if self.props.ripple {
            self.ripple = window()
                .and_then(|w| w.document())
                .and_then(|d| d.get_element_by_id(&self.props.id))
                .map(|elem| MDCRipple::new(elem));
            true
        } else {
            false
        }
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
        html! {
            <button class=format!("mdc-button {}", self.props.style)
                    id=self.props.id
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
