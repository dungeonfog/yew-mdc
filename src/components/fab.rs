use crate::mdc_sys::MDCRipple;
use yew::prelude::*;

pub struct FAB {
    id: String,
    ripple: Option<MDCRipple>,
    props: Props,
    link: ComponentLink<Self>,
}

#[derive(Properties, Clone, PartialEq)]
pub struct Props {
    pub children: Children,
    #[prop_or_default]
    pub id: Option<String>,
    #[prop_or_default]
    pub text: Option<String>,
    #[prop_or_default]
    pub mini: bool,
    #[prop_or_default]
    pub exited: bool,
    #[prop_or_else(Callback::noop)]
    pub onclick: Callback<()>,
}

pub enum Msg {
    Clicked,
}

impl Component for FAB {
    type Message = Msg;
    type Properties = Props;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        let id = props
            .id
            .as_ref()
            .map(|id| id.to_owned())
            .unwrap_or_else(|| format!("fab-{}", crate::next_id()));
        Self {
            id,
            ripple: None,
            props,
            link,
        }
    }

    fn change(&mut self, props: Props) -> ShouldRender {
        if self.props != props {
            self.props = props;
            true
        } else {
            false
        }
    }

    fn mounted(&mut self) -> ShouldRender {
        self.ripple = crate::get_element_by_id(&self.id).map(MDCRipple::new);
        false
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::Clicked => {
                self.props.onclick.emit(());
            }
        }
        false
    }

    fn view(&self) -> Html {
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
        let onclick = self.link.callback(|_| Msg::Clicked);
        html! {
            <button class=classes id=self.id onclick=onclick>
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
