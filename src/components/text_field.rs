use crate::mdc_sys::MDCTextField;
use wasm_bindgen::JsCast;
use web_sys::{Element, EventTarget, HtmlInputElement};
use yew::prelude::*;

pub mod helper_line;
pub use helper_line::HelperLine;

pub mod text_area;

pub struct TextField {
    node_ref: NodeRef,
    inner: Option<MDCTextField>,
}

#[derive(PartialEq, Properties, Clone, Debug)]
pub struct Props {
    #[prop_or_default]
    pub id: String,
    #[prop_or_default]
    pub classes: String,
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
    #[prop_or_else(Callback::noop)]
    pub onkeydown: Callback<KeyboardEvent>,
    #[prop_or_default]
    pub children: Children,
    #[prop_or_default]
    pub evil_gimme_focus_callback: Option<Callback<Callback<()>>>,
}

pub enum Msg {
    ValueChanged(String),
    KeyDown(KeyboardEvent),
    FocusRequested,
}

impl Component for TextField {
    type Message = Msg;
    type Properties = Props;

    fn create(ctx: &Context<Self>) -> Self {
        if let Some(ref callback) = ctx.props().evil_gimme_focus_callback {
            let my_callback = ctx.link().callback(|_| Msg::FocusRequested);
            callback.emit(my_callback);
        }
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
        if let Some(ref callback) = ctx.props().evil_gimme_focus_callback {
            let my_callback = ctx.link().callback(|_| Msg::FocusRequested);
            callback.emit(my_callback);
        }
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
            Msg::KeyDown(e) => {
                ctx.props().onkeydown.emit(e);
            }
            Msg::FocusRequested => {
                if let Some(ref inner) = self.inner {
                    inner.focus();
                }
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
        let outlined = if ctx.props().outlined {
            " mdc-text-field--outlined"
        } else {
            ""
        };
        let nolabel = if ctx.props().nolabel {
            " mdc-text-field--no-label"
        } else {
            ""
        };
        let classes = format!(
            "mdc-text-field{}{}{} {}",
            disabled,
            outlined,
            nolabel,
            ctx.props().classes
        );
        let label = if ctx.props().nolabel {
            html! {}
        } else {
            html! {
                <label class="mdc-floating-label">
                    { &ctx.props().hint }
                </label>
            }
        };
        let inner = if ctx.props().outlined {
            let notch = if ctx.props().nolabel {
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
        let placeholder = if ctx.props().nolabel && !ctx.props().hint.is_empty() {
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
                { ctx.props().children.clone() }
                <input type="text"
                       value={ctx.props().value.clone()}
                       class="mdc-text-field__input"
                       oninput={oninput}
                       onkeydown={ctx.link().callback(Msg::KeyDown)}
                       disabled={ctx.props().disabled}
                       placeholder={placeholder}
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
