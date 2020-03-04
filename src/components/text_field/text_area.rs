use crate::mdc_sys::MDCTextField;
use web_sys::Element;
use yew::prelude::*;

pub struct TextArea {
    node_ref: NodeRef,
    inner: Option<MDCTextField>,
    props: Props,
    link: ComponentLink<Self>,
}

#[derive(PartialEq, Properties, Clone, Debug)]
pub struct Props {
    #[prop_or_default]
    pub id: String,
    #[prop_or_default]
    pub value: String,
    #[prop_or_default]
    pub hint: String,
    #[prop_or_else(Callback::noop)]
    pub onchange: Callback<String>,
    #[prop_or_default]
    pub disabled: bool,
    #[prop_or_default]
    pub nolabel: bool,
    #[prop_or_default]
    pub rows: Option<u32>,
    #[prop_or_default]
    pub cols: Option<u32>,
}

pub enum Msg {
    ValueChanged(String),
}

impl Component for TextArea {
    type Message = Msg;
    type Properties = Props;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self {
            node_ref: NodeRef::default(),
            props,
            inner: None,
            link,
        }
    }

    fn mounted(&mut self) -> ShouldRender {
        self.inner = self.node_ref.cast::<Element>().map(MDCTextField::new);
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
                self.props.onchange.emit(s);
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
                        <label class="mdc-floating-label">
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
            <div class=classes id=&self.props.id ref=self.node_ref.clone()>
                <textarea value=self.props.value
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
