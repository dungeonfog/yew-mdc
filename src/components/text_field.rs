use crate::mdc_sys::MDCTextField;
use yew::prelude::*;

// pub mod line_ripple;
// pub use line_ripple::LineRipple;
// pub mod floating_label;
// pub use floating_label::FloatingLabel;

pub struct TextField {
    pub id: String,
    pub input_id: String,
    inner: Option<MDCTextField>,
    props: Props,
}

#[derive(PartialEq, Properties, Debug)]
pub struct Props {
    pub id: Option<String>,
    pub value: String,
    pub hint: String,
    pub onchange: Option<Callback<String>>,
}

pub enum Msg {
    ValueChanged(String),
}

impl Component for TextField {
    type Message = Msg;
    type Properties = Props;

    fn create(props: Self::Properties, _link: ComponentLink<Self>) -> Self {
        let id = props
            .id
            .as_ref()
            .map(|s| s.to_owned())
            .unwrap_or_else(|| format!("text-field-{}", crate::next_id()));
        let input_id = format!("{}-input", id);
        Self {
            id,
            input_id,
            props,
            inner: None,
        }
    }

    fn mounted(&mut self) -> ShouldRender {
        self.inner = crate::get_element_by_id(&self.id).map(MDCTextField::new);
        false
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::ValueChanged(s) => {
                if let Some(callback) = &self.props.onchange {
                    callback.emit(s);
                }
            }
        };
        false
    }

    fn view(&self) -> Html<Self> {
        html! {
            <div class="mdc-text-field" id=self.id>
                <input type="text" id=self.input_id
                       value=self.props.value
                       class="mdc-text-field__input"
                       oninput=|e| Msg::ValueChanged(e.value)
                    />
                <div class="mdc-line-ripple"></div>
                <label class="mdc-floating-label" for=self.input_id>
                    { &self.props.hint }
                </label>
            </div>
        }
    }

    fn destroy(&mut self) {
        if let Some(inner) = &self.inner {
            inner.destroy();
        }
    }
}
