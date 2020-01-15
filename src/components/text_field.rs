use crate::mdc_sys::MDCTextField;
use yew::prelude::*;

pub mod helper_line;
pub use helper_line::HelperLine;

pub struct TextField {
    pub id: String,
    pub input_id: String,
    inner: Option<MDCTextField>,
    props: Props,
    link: ComponentLink<Self>,
}

#[derive(PartialEq, Properties, Clone, Debug)]
pub struct Props {
    pub id: Option<String>,
    pub value: String,
    pub hint: String,
    pub onchange: Option<Callback<String>>,
    pub disabled: bool,
    pub outlined: bool,
    pub nolabel: bool,
}

pub enum Msg {
    ValueChanged(String),
}

impl Component for TextField {
    type Message = Msg;
    type Properties = Props;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
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
            link,
        }
    }

    fn mounted(&mut self) -> ShouldRender {
        self.inner = crate::get_element_by_id(&self.id).map(MDCTextField::new);
        false
    }

    fn change(&mut self, props: Props) -> ShouldRender {
        if props != self.props {
            self.props = props;
            if let Some(inner) = &self.inner {
                inner.set_value(&self.props.value);
            }
            true
        } else {
            false
        }
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

    fn view(&self) -> Html {
        let disabled = if self.props.disabled {
            " mdc-text-field--disabled"
        } else {
            ""
        };
        let outlined = if self.props.outlined {
            " mdc-text-field--outlined"
        } else {
            ""
        };
        let nolabel = if self.props.nolabel {
            " mdc-text-field--no-label"
        } else {
            ""
        };
        let classes = format!("mdc-text-field{}{}{}", disabled, outlined, nolabel);
        let label = if self.props.nolabel {
            html! {}
        } else {
            html! {
                <label class="mdc-floating-label" for=self.input_id>
                    { &self.props.hint }
                </label>
            }
        };
        let inner = if self.props.outlined {
            let notch = if self.props.nolabel {
                html! {}
            } else {
                html! {
                    <div class="mdc-notched-outline__notch">
                        { label }
                    </div>
                }
            };
            html! {
                <div class="mdc-notched-outline">
                    <div class="mdc-notched-outline__leading"></div>
                    { notch }
                    <div class="mdc-notched-outline__trailing"></div>
                </div>
            }
        } else {
            html! { <>
                <div class="mdc-line-ripple"></div>
                { label }
            </> }
        };
        let placeholder = if self.props.nolabel && !self.props.hint.is_empty() {
            &self.props.hint
        } else {
            ""
        };
        let oninput = self
            .link
            .callback(|e: InputData| Msg::ValueChanged(e.value));
        html! {
            <div class=classes id=self.id>
                <input type="text" id=self.input_id
                       value=self.props.value
                       class="mdc-text-field__input"
                       oninput=oninput
                       disabled=self.props.disabled
                       placeholder=placeholder
                    />
                { inner }
            </div>
        }
    }

    fn destroy(&mut self) {
        if let Some(inner) = &self.inner {
            inner.destroy();
        }
    }
}
