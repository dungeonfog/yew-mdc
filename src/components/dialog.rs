use crate::mdc_sys::MDCDialog;
use wasm_bindgen::{prelude::*, JsCast};
use yew::prelude::*;

pub mod actions;
pub use actions::Actions;
pub mod content;
pub use content::Content;

pub struct Dialog {
    id: String,
    inner: Option<MDCDialog>,
    close_callback: Closure<dyn FnMut(web_sys::Event)>,
    props: Props,
}

#[derive(Properties, Clone)]
pub struct Props {
    pub id: Option<String>,
    pub children: Children,
    pub onclosed: Option<Callback<Option<String>>>,
    pub escape_key_action: Option<String>,
    pub scrim_click_action: Option<String>,
    pub auto_stack_buttons: bool,
    pub title: String,
    pub open: bool,
}

pub enum Msg {
    Closed { action: Option<String> },
}

impl Component for Dialog {
    type Message = Msg;
    type Properties = Props;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        let id = props
            .id
            .as_ref()
            .map(|s| s.to_owned())
            .unwrap_or_else(|| format!("dialog-{}", crate::next_id()));
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
            id,
            inner: None,
            close_callback: closure,
            props,
        }
    }

    fn mounted(&mut self) -> ShouldRender {
        if let Some(elem) = crate::get_element_by_id(&self.id) {
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
        false
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
        self.props = props;
        true
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::Closed { action } => {
                if let Some(callback) = &self.props.onclosed {
                    callback.emit(action);
                }
            }
        }
        false
    }

    fn view(&self) -> Html {
        html! {
            <div class="mdc-dialog" id=self.id>
                <div class="mdc-dialog__container">
                    <div class="mdc-dialog__surface">
                        <h2 class="mdc-dialog__title">{ &self.props.title }</h2>
                        { self.props.children.render() }
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
