use crate::mdc_sys::MDCTextField;
use web_sys::Element;
use yew::prelude::*;

pub mod helper_line;
pub use helper_line::HelperLine;

pub mod text_area;

pub struct TextField {
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
    #[prop_or_default]
    pub disabled: bool,
    #[prop_or_default]
    pub outlined: bool,
    #[prop_or_default]
    pub nolabel: bool,
    #[prop_or_else(Callback::noop)]
    pub onchange: Callback<String>,
}

pub enum Msg {
    ValueChanged(String),
}

impl Component for TextField {
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
                <label class="mdc-floating-label">
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
            <div class=classes id=&self.props.id ref=self.node_ref.clone()>
                <input type="text"
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
