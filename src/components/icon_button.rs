use crate::mdc_sys::MDCRipple;
use yew::prelude::*;

pub struct IconButton {
    id: String,
    ripple: Option<MDCRipple>,
    props: Props,
}

#[derive(Properties)]
pub struct Props {
    pub id: Option<String>,
    pub classes: String,
    pub children: Children<IconButton>,
    pub togglable: bool,
    pub onclick: Option<Callback<()>>,
}

pub enum Msg {
    Clicked,
}

impl Component for IconButton {
    type Message = Msg;
    type Properties = Props;

    fn create(props: Self::Properties, _link: ComponentLink<Self>) -> Self {
        let id = props
            .id
            .as_ref()
            .map(|id| id.to_owned())
            .unwrap_or_else(|| format!("icon-button-{}", crate::next_id()));
        Self {
            id,
            ripple: None,
            props,
        }
    }

    fn mounted(&mut self) -> ShouldRender {
        self.ripple = crate::get_element_by_id(&self.id).map(|elem| {
            let ripple = MDCRipple::new(elem);
            ripple.set_unbounded(true);
            ripple
        });
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
        if self.props.togglable {
            html! {}
        } else {
            let classes = format!("mdc-icon-button {}", self.props.classes);
            html! {
                <button class=classes
                        onclick=|_| Msg::Clicked
                    >{ self.props.children.render() }</button>
            }
        }
    }

    fn destroy(&mut self) {}
}
