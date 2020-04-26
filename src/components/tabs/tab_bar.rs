use crate::mdc_sys::MDCTabBar;
use wasm_bindgen::{prelude::*, JsCast};
use yew::prelude::*;

pub struct TabBar {
    props: Props,
    inner: Option<MDCTabBar>,
    node_ref: NodeRef,
    activated_callback: Closure<dyn FnMut(web_sys::Event)>,
    current_tab: u64,
}

#[derive(Properties, Clone, PartialEq)]
pub struct Props {
    pub children: Children,

    #[prop_or_default]
    pub id: String,

    /// If set to true (default), tabs are focused on activation.
    #[prop_or(true)]
    pub focus_tabs_on_activate: bool,
    /// If set to true (default), tabs get activated when focused with the arrow keys.
    /// If false, they only get activated on enter/space bar press.
    #[prop_or(true)]
    pub arrow_key_tab_activation: bool,

    /// Tab to activate after rendering.
    #[prop_or_default]
    pub activated_tab: Option<u32>,

    #[prop_or_else(Callback::noop)]
    pub ontabactivate: Callback<u64>,
}

pub enum Msg {
    TabActivated(u64),
}

impl Component for TabBar {
    type Message = Msg;
    type Properties = Props;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        let callback = link.callback(Msg::TabActivated);
        let closure = Closure::wrap(Box::new(move |e: web_sys::Event| {
            if let Some(e) = e.dyn_ref::<web_sys::CustomEvent>() {
                e.stop_propagation();
                if let Ok(value) = e.detail().into_serde::<serde_json::Value>() {
                    if let Some(index) = value.get("index").and_then(|v| v.as_u64()) {
                        callback.emit(index);
                    }
                }
            }
            e.stop_propagation();
        }) as Box<dyn FnMut(web_sys::Event)>);
        Self {
            props,
            inner: None,
            node_ref: NodeRef::default(),
            activated_callback: closure,
            current_tab: 0,
        }
    }

    fn mounted(&mut self) -> ShouldRender {
        if let Some(tab_bar) = self.node_ref.cast::<web_sys::Element>().map(MDCTabBar::new) {
            tab_bar.focus_on_activate(self.props.focus_tabs_on_activate);
            tab_bar.use_automatic_activation(self.props.arrow_key_tab_activation);
            tab_bar.listen("MDCTabBar:activated", &self.activated_callback);
            if let Some(index) = self.props.activated_tab {
                self.current_tab = index as u64;
                tab_bar.activate_tab(index);
            }
            self.inner = Some(tab_bar);
        }
        false
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        if self.props != props {
            if let Some(tab) = props.activated_tab {
                if tab as u64 != self.current_tab {
                    if let Some(ref inner) = self.inner {
                        inner.activate_tab(tab);
                    }
                }
            }
            self.props = props;
            true
        } else {
            false
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::TabActivated(index) => {
                self.current_tab = index;
                self.props.ontabactivate.emit(index);
            }
        }
        false
    }

    fn view(&self) -> Html {
        html! {
            <div class="mdc-tab-bar"
                 ref=self.node_ref.clone()
                 id=&self.props.id
                >
                { self.props.children.render() }
            </div>
        }
    }

    fn destroy(&mut self) {
        if let Some(ref inner) = self.inner {
            inner.unlisten("MDCTabBar:activated", &self.activated_callback);
            inner.destroy();
        }
    }
}
