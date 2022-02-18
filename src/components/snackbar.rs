use crate::mdc_sys::MDCSnackbar;
use wasm_bindgen::closure::Closure;
use web_sys::Element;
use yew::prelude::*;

use crate::components::Button;

pub struct Snackbar {
    node_ref: NodeRef,
    inner: Option<MDCSnackbar>,
    close_callback: Closure<dyn FnMut(web_sys::CustomEvent)>,
}

#[derive(Properties, Clone, PartialEq)]
pub struct Props {
    #[prop_or_default]
    pub id: String,
    #[prop_or_default]
    pub text: String,
    #[prop_or_default]
    pub action_text: String,
    #[prop_or_default]
    pub onactionclicked: Option<Callback<MouseEvent>>,
    #[prop_or_default]
    pub onclose: Option<Callback<()>>,
    #[prop_or_default]
    pub timeout_ms: Option<i16>,
    pub open: bool,
}

pub enum Msg {
    ActionClicked(MouseEvent),
    Closed,
}

impl Component for Snackbar {
    type Message = Msg;
    type Properties = Props;

    fn create(ctx: &Context<Self>) -> Self {
        let close_callback = {
            let callback = ctx.link().callback(|_| Msg::Closed);
            Closure::wrap(Box::new(move |e: web_sys::CustomEvent| {
                e.stop_propagation();
                callback.emit(());
            }) as Box<dyn FnMut(web_sys::CustomEvent)>)
        };
        Self {
            node_ref: NodeRef::default(),
            inner: None,
            close_callback,
        }
    }

    fn rendered(&mut self, ctx: &Context<Self>, first_render: bool) {
        if first_render {
            if let Some(inner) = self.inner.take() {
                inner.unlisten("MDCSnackbar:closed", &self.close_callback);
                inner.destroy();
            }
            if let Some(elem) = self.node_ref.cast::<Element>() {
                let inner = MDCSnackbar::new(elem);
                if let Some(timeout_ms) = &ctx.props().timeout_ms {
                    inner.set_timeout_ms(*timeout_ms);
                }
                if ctx.props().open {
                    inner.open();
                }
                inner.listen("MDCSnackbar:closed", &self.close_callback);
                self.inner = Some(inner);
            }
        }
    }

    fn changed(&mut self, ctx: &Context<Self>) -> bool {
        if let Some(ref inner) = self.inner {
            if ctx.props().open {
                inner.open();
            } else {
                inner.close(None);
            }
        }
        true
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::ActionClicked(ev) => {
                if let Some(ref callback) = ctx.props().onactionclicked {
                    callback.emit(ev);
                }
            }
            Msg::Closed => {
                if let Some(ref callback) = ctx.props().onclose {
                    callback.emit(());
                }
            }
        }
        false
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let actions = if !ctx.props().action_text.is_empty() {
            let emit_action = ctx.link().callback(Msg::ActionClicked);
            html! {
                <div class="mdc-snackbar__actions">
                    <Button text={ctx.props().action_text.clone()}
                            classes="mdc-snackbar__action"
                            onclick={emit_action}
                            />
                </div>
            }
        } else {
            html! {}
        };
        html! {
            <div class="mdc-snackbar"
                 id={ctx.props().id.clone()}
                 ref={self.node_ref.clone()}>
                <div class="mdc-snackbar__surface">
                    <div class="mdc-snackbar__label">
                        { &ctx.props().text }
                    </div>
                    { actions }
                </div>
            </div>
        }
    }

    fn destroy(&mut self, _ctx: &Context<Self>) {
        if let Some(ref inner) = self.inner {
            inner.unlisten("MDCSnackbar:closed", &self.close_callback);
            inner.destroy();
        }
    }
}
