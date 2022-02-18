use crate::mdc_sys::MDCTextField;
use wasm_bindgen::JsCast;
use web_sys::{Element, EventTarget, HtmlInputElement};
use yew::events::InputEvent;
use yew::prelude::*;

pub struct TextArea {
    node_ref: NodeRef,
    inner: Option<MDCTextField>,
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

    fn create(_ctx: &Context<Self>) -> Self {
        Self {
            node_ref: NodeRef::default(),
            inner: None,
        }
    }

    fn rendered(&mut self, _ctx: &Context<Self>, first_render: bool) {
        if first_render {
            if let Some(inner) = self.inner.take() {
                inner.destroy();
            }
            self.inner = self.node_ref.cast::<Element>().map(MDCTextField::new);
        }
    }

    fn changed(&mut self, ctx: &Context<Self>) -> bool {
        if let Some(inner) = &self.inner {
            inner.set_value(&ctx.props().value);
        }
        true
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::ValueChanged(s) => {
                ctx.props().onchange.emit(s);
            }
        };
        false
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let disabled = if ctx.props().disabled {
            " mdc-text-field--disabled"
        } else {
            ""
        };
        let nolabel = if ctx.props().nolabel {
            " mdc-text-field--no-label"
        } else {
            ""
        };
        let classes = format!(
            "mdc-text-field mdc-text-field--textarea{}{}",
            disabled, nolabel
        );
        let inner = {
            let notch = if ctx.props().nolabel {
                html! {}
            } else {
                html! {
                    <div class="mdc-notched-outline__notch">
                        <label class="mdc-floating-label">
                            { &ctx.props().hint }
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
        let placeholder = if ctx.props().nolabel {
            ctx.props().hint.clone()
        } else {
            "".to_string()
        };
        let oninput = ctx.link().batch_callback(|e: InputEvent| {
            let target: Option<EventTarget> = e.target();
            let input = target.and_then(|t| t.dyn_into::<HtmlInputElement>().ok());

            input.map(|input| Msg::ValueChanged(input.value()))
        });
        html! {
            <div class={classes} id={ctx.props().id.clone()} ref={self.node_ref.clone()}>
                <textarea value={ctx.props().value.clone()}
                          class="mdc-text-field__input"
                          oninput={oninput}
                          disabled={ctx.props().disabled}
                          placeholder={placeholder}
                          cols={ctx.props().cols.unwrap_or(80).to_string()}
                          rows={ctx.props().rows.unwrap_or(4).to_string()}
                    />
                { inner }
            </div>
        }
    }

    fn destroy(&mut self, _ctx: &Context<Self>) {
        if let Some(inner) = &self.inner {
            inner.destroy();
        }
    }
}
