use crate::mdc_sys::MDCTextField;
use yew::prelude::*;

pub struct TextArea {
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
    pub nolabel: bool,
    pub rows: Option<u32>,
    pub cols: Option<u32>,
}

pub enum Msg {
    ValueChanged(String),
}

impl Component for TextArea {
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
        let nolabel = if self.props.nolabel {
            " mdc-text-field--no-label"
        } else {
            ""
        };
        let classes = format!(
            "mdc-text-field mdc-text-field--textarea{}{}",
            disabled, nolabel
        );
        let inner = {
            let notch = if self.props.nolabel {
                html! {}
            } else {
                html! {
                    <div class="mdc-notched-outline__notch">
                        <label class="mdc-floating-label" for=self.input_id>
                            { &self.props.hint }
                        </label>
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
        };
        let placeholder = if self.props.nolabel {
            &self.props.hint
        } else {
            ""
        };
        let oninput = self
            .link
            .callback(|e: InputData| Msg::ValueChanged(e.value));
        html! {
            <div class=classes id=self.id>
                <textarea id=self.input_id
                          value=self.props.value
                          class="mdc-text-field__input"
                          oninput=oninput
                          disabled=self.props.disabled
                          placeholder=placeholder
                          cols=self.props.cols.unwrap_or(80)
                          rows=self.props.rows.unwrap_or(4)
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
