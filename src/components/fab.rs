use crate::mdc_sys::MDCRipple;
use yew::prelude::*;

pub struct FAB {
    id: String,
    ripple: Option<MDCRipple>,
    props: Props,
}

#[derive(Properties)]
pub struct Props {
    pub id: Option<String>,
    pub children: Children<FAB>,
    pub text: Option<String>,
    pub mini: bool,
    pub exited: bool,
    pub onclick: Option<Callback<()>>,
}

pub enum Msg {
    Clicked,
}

impl Component for FAB {
    type Message = Msg;
    type Properties = Props;

    fn create(props: Self::Properties, _link: ComponentLink<Self>) -> Self {
        let id = props
            .id
            .as_ref()
            .map(|id| id.to_owned())
            .unwrap_or_else(|| format!("fab-{}", crate::next_id()));
        Self {
            id,
            ripple: None,
            props,
        }
    }

    fn mounted(&mut self) -> ShouldRender {
        self.ripple = crate::get_element_by_id(&self.id).map(MDCRipple::new);
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

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        if props.exited != self.props.exited {
            self.props = props;
            true
        } else {
            false
        }
    }

    fn view(&self) -> Html<Self> {
        let ripple = html! { <div class="mdc-fab__ripple"></div> };
        let mini = if self.props.mini {
            " mdc-fab--mini"
        } else {
            ""
        };
        let exited = if self.props.exited {
            " mdc-fab--exited"
        } else {
            ""
        };
        let (label, extended) = if let Some(text) = &self.props.text {
            (
                html! { <span class="mdc-fab__label">{ text }</span> },
                " mdc-fab--extended",
            )
        } else {
            (html! {}, "")
        };
        let classes = format!("mdc-fab{}{}{}", mini, extended, exited);
        html! {
            <button class=classes id=self.id>
                { ripple }
                { self.props.children.render() }
                { label }
            </button>
        }
    }

    fn destroy(&mut self) {
        if let Some(ripple) = &self.ripple {
            ripple.destroy();
        }
    }
}
