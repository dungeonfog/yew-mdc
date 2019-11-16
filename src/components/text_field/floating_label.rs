use crate::mdc_sys::MDCFloatingLabel;
use web_sys::window;
use yew::prelude::*;

pub struct FloatingLabel {
    id: String,
    text: String,
    for_id: Option<String>,
    inner: Option<MDCFloatingLabel>,
}

#[derive(PartialEq, Properties, Debug)]
pub struct Props {
    pub id: Option<String>,
    pub for_id: Option<String>,
    #[props(required)]
    pub text: String,
}

pub enum Msg {
    Float(bool),
    Shake(bool),
}

impl Component for FloatingLabel {
    type Message = Msg;
    type Properties = Props;

    fn create(props: Self::Properties, _link: ComponentLink<Self>) -> Self {
        let Props { id, for_id, text } = props;
        Self {
            id: id.unwrap_or_else(|| format!("floating-label-{}", crate::next_id())),
            inner: None,
            for_id,
            text,
        }
    }

    fn mounted(&mut self) -> ShouldRender {
        log::trace!(
            "Mounting floating label (id: {:?}, for_id: {:?}, text: {:?})",
            self.id,
            self.for_id,
            self.text
        );
        self.inner = window()
            .and_then(|w| w.document())
            .and_then(|d| d.get_element_by_id(&self.id))
            .map(MDCFloatingLabel::new);
        false
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        if let Some(inner) = &self.inner {
            match msg {
                Msg::Float(should) => inner.float(should),
                Msg::Shake(should) => inner.shake(should),
            };
        }
        false
    }

    fn view(&self) -> Html<Self> {
        // If an id was assigned, use it, assuming we're wrapped by a div.
        // Otherwise, assume we're wrapped by a label.
        if let Some(for_id) = &self.for_id {
            html! {
                <label class="mdc-floating-label" for=for_id>
                    { &self.text }
                </label>
            }
        } else {
            html! {
                <span class="mdc-floating-label">{ &self.text }</span>
            }
        }
    }

    fn destroy(&mut self) {
        if let Some(inner) = &self.inner {
            inner.destroy();
        }
    }
}
