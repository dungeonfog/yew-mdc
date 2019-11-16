use crate::mdc_sys::MDCLineRipple;
use web_sys::window;
use yew::prelude::*;

pub struct LineRipple {
    id: String,
    inner: Option<MDCLineRipple>,
}

#[derive(PartialEq, Properties)]
pub struct Props {
    pub id: Option<String>,
}

impl Component for LineRipple {
    type Message = ();
    type Properties = Props;

    fn create(props: Self::Properties, _link: ComponentLink<Self>) -> Self {
        let id = props
            .id
            .unwrap_or_else(|| format!("line-ripple-{}", crate::next_id()));
        Self { id, inner: None }
    }

    fn mounted(&mut self) -> ShouldRender {
        log::trace!("Mounting line ripple {}", self.id);
        self.inner = window()
            .and_then(|w| w.document())
            .and_then(|d| d.get_element_by_id(&self.id))
            .map(MDCLineRipple::new);
        false
    }

    fn update(&mut self, _msg: Self::Message) -> ShouldRender {
        false
    }

    fn view(&self) -> Html<Self> {
        html! {
            <div class="mdc-line-ripple" id=self.id></div>
        }
    }

    fn destroy(&mut self) {
        if let Some(inner) = &self.inner {
            inner.destroy();
        }
    }
}
