use crate::mdc_sys::MDCSnackbar;
use wasm_bindgen::closure::Closure;
use web_sys::Element;
use yew::prelude::*;

use crate::components::Button;

pub struct Snackbar {
    node_ref: NodeRef,
    link: ComponentLink<Self>,
    inner: Option<MDCSnackbar>,
    close_callback: Closure<dyn FnMut(web_sys::Event)>,
    props: Props,
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

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        let close_callback = {
            let callback = link.callback(|_| Msg::Closed);
            Closure::wrap(Box::new(move |e: web_sys::Event| {
                e.stop_propagation();
                callback.emit(());
            }) as Box<dyn FnMut(web_sys::Event)>)
        };
        Self {
            node_ref: NodeRef::default(),
            link,
            inner: None,
            close_callback,
            props,
        }
    }

    fn rendered(&mut self, first_render: bool) {
        if first_render {
            if let Some(inner) = self.inner.take() {
                inner.unlisten("MDCSnackbar:closed", &self.close_callback);
                inner.destroy();
            }
            if let Some(elem) = self.node_ref.cast::<Element>() {
                let inner = MDCSnackbar::new(elem);
                if let Some(timeout_ms) = &self.props.timeout_ms {
                    inner.set_timeout_ms(*timeout_ms);
                }
                if self.props.open {
                    inner.open();
                }
                inner.listen("MDCSnackbar:closed", &self.close_callback);
                self.inner = Some(inner);
            }
        }
    }

    fn change(&mut self, props: Props) -> ShouldRender {
        if self.props != props {
            self.props = props;
            if let Some(ref inner) = self.inner {
                if self.props.open {
                    inner.open();
                } else {
                    inner.close(None);
                }
            }
            true
        } else {
            false
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::ActionClicked(ev) => {
                if let Some(ref callback) = self.props.onactionclicked {
                    callback.emit(ev);
                }
            }
            Msg::Closed => {
                if let Some(ref callback) = self.props.onclose {
                    callback.emit(());
                }
            }
        }
        false
    }

    fn view(&self) -> Html {
        let actions = if !self.props.action_text.is_empty() {
            let emit_action = self.link.callback(Msg::ActionClicked);
            html! {
                <div class="mdc-snackbar__actions">
                    <Button text=self.props.action_text.clone()
                            classes="mdc-snackbar__action"
                            onclick=emit_action
                            />
                </div>
            }
        } else {
            html! {}
        };
        html! {
            <div class="mdc-snackbar"
                 id=self.props.id.clone()
                 ref=self.node_ref.clone()>
                <div class="mdc-snackbar__surface">
                    <div class="mdc-snackbar__label">
                        { &self.props.text }
                    </div>
                    { actions }
                </div>
            </div>
        }
    }

    fn destroy(&mut self) {
        if let Some(ref inner) = self.inner {
            inner.unlisten("MDCSnackbar:closed", &self.close_callback);
            inner.destroy();
        }
    }
}
