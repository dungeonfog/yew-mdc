use crate::mdc_sys::MDCDialog;
use wasm_bindgen::{prelude::*, JsCast};
use web_sys::Element;
use yew::prelude::*;

pub mod actions;
pub use actions::Actions;
pub mod content;
pub use content::Content;

pub struct Dialog {
    node_ref: NodeRef,
    inner: Option<MDCDialog>,
    close_callback: Closure<dyn FnMut(web_sys::Event)>,
    props: Props,
    link: ComponentLink<Self>,
}

#[derive(Properties, Clone, PartialEq)]
pub struct Props {
    pub children: Children,
    #[prop_or_default]
    pub id: String,
    #[prop_or_else(Callback::noop)]
    pub onclosed: Callback<Option<String>>,
    #[prop_or_default]
    pub escape_key_action: Option<String>,
    #[prop_or_default]
    pub scrim_click_action: Option<String>,
    #[prop_or_default]
    pub auto_stack_buttons: bool,
    #[prop_or_default]
    pub title: String,
    #[prop_or_default]
    pub open: bool,
    #[prop_or_default]
    pub onkeydown: Callback<KeyboardEvent>,
}

pub enum Msg {
    Closed { action: Option<String> },
    KeyDown(KeyboardEvent),
}

impl Component for Dialog {
    type Message = Msg;
    type Properties = Props;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        let callback = link.callback(|action: Option<String>| Msg::Closed { action });
        let closure = Closure::wrap(Box::new(move |e: web_sys::Event| {
            use std::borrow::ToOwned;
            e.stop_propagation();
            let action = e.dyn_ref::<web_sys::CustomEvent>().and_then(|e| {
                e.detail()
                    .into_serde::<serde_json::Value>()
                    .ok()
                    .and_then(|v| {
                        v.get("action")
                            .and_then(|v| v.as_str())
                            .map(ToOwned::to_owned)
                    })
            });
            callback.emit(action);
        }) as Box<dyn FnMut(web_sys::Event)>);
        Self {
            node_ref: NodeRef::default(),
            inner: None,
            close_callback: closure,
            props,
            link,
        }
    }

    fn rendered(&mut self, first_render: bool) {
        if first_render {
            if let Some(old_inner) = self.inner.take() {
                old_inner.unlisten("MDCDialog:closed", &self.close_callback);
                old_inner.destroy();
            }
            if let Some(elem) = self.node_ref.cast::<Element>() {
                let dialog = MDCDialog::new(elem);
                if let Some(action) = &self.props.escape_key_action {
                    dialog.set_escape_key_action(action);
                }
                if let Some(action) = &self.props.scrim_click_action {
                    dialog.set_scrim_click_action(action);
                }
                dialog.set_auto_stack_buttons(self.props.auto_stack_buttons);
                dialog.listen("MDCDialog:closed", &self.close_callback);
                if self.props.open {
                    dialog.open();
                }
                self.inner = Some(dialog);
            }
        }
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        if props.open != self.props.open {
            if let Some(inner) = &self.inner {
                if props.open {
                    inner.open();
                } else {
                    inner.close(None);
                }
            }
        }
        if self.props != props {
            self.props = props;
            true
        } else {
            false
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::Closed { action } => {
                self.props.onclosed.emit(action);
            }
            Msg::KeyDown(ev) => self.props.onkeydown.emit(ev),
        }
        false
    }

    fn view(&self) -> Html {
        html! {
            <div
                class="mdc-dialog"
                id=self.props.id.clone()
                ref=self.node_ref.clone()
                onkeydown=self.link.callback(Msg::KeyDown)>
                <div class="mdc-dialog__container">
                    <div class="mdc-dialog__surface">
                        <h2 class="mdc-dialog__title">{ &self.props.title }</h2>
                        { self.props.children.clone() }
                    </div>
                </div>
                <div class="mdc-dialog__scrim"></div>
            </div>
        }
    }

    fn destroy(&mut self) {
        if let Some(inner) = &self.inner {
            inner.unlisten("MDCDialog:closed", &self.close_callback);
            inner.destroy();
        }
    }
}
