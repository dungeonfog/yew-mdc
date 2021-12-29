use crate::mdc_sys::{MDCMenu, MDCMenuSurface};
use wasm_bindgen::prelude::*;
use web_sys::Element;
use yew::prelude::*;

pub mod item;
pub use item::Item;

pub struct Menu {
    node_ref: NodeRef,
    inner: Option<MDCMenu>,
    surface: Option<MDCMenuSurface>,
    close_callback: Closure<dyn FnMut(web_sys::Event)>,
    props: Props,
}

#[derive(Properties, Clone, PartialEq)]
pub struct Props {
    pub children: Children,
    #[prop_or_default]
    pub id: String,
    #[prop_or_default]
    pub fixed_position: bool,
    #[prop_or_default]
    pub absolute_position: Option<(i32, i32)>,
    #[prop_or_default]
    pub open: bool,
    #[prop_or_else(Callback::noop)]
    pub onclose: Callback<()>,
}

pub enum Msg {
    Closed,
}

impl Component for Menu {
    type Message = Msg;
    type Properties = Props;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        let callback = link.callback(|_| Msg::Closed);
        let closure = Closure::wrap(Box::new(move |e: web_sys::Event| {
            e.stop_propagation();
            callback.emit(());
        }) as Box<dyn FnMut(web_sys::Event)>);
        Self {
            node_ref: NodeRef::default(),
            inner: None,
            surface: None,
            close_callback: closure,
            props,
        }
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        if let Some(inner) = &self.inner {
            if props.open != self.props.open {
                self.props.open = props.open;
                inner.set_open(props.open);
            }
            if props.fixed_position != self.props.fixed_position {
                self.props.fixed_position = props.fixed_position;
                inner.set_fixed_position(self.props.fixed_position);
            }
            if props.absolute_position != self.props.absolute_position {
                self.props.absolute_position = props.absolute_position;
                if let Some((x, y)) = props.absolute_position {
                    inner.set_absolute_position(x, y);
                    inner.set_is_hoisted(true);
                } else {
                    inner.set_absolute_position(0, 0);
                    inner.set_is_hoisted(false);
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

    fn rendered(&mut self, first_render: bool) {
        if first_render {
            if let Some(surface) = self.surface.take() {
                surface.unlisten("MDCMenuSurface:closed", &self.close_callback);
                surface.destroy();
            }
            if let Some(inner) = self.inner.take() {
                inner.destroy();
            }
            if let Some(elem) = self.node_ref.cast::<Element>() {
                // Our root element has the mdc-menu class...
                let menu = MDCMenu::new(elem.clone());
                menu.set_fixed_position(self.props.fixed_position);
                if let Some((x, y)) = self.props.absolute_position {
                    menu.set_absolute_position(x, y);
                }
                menu.set_open(self.props.open);
                self.inner = Some(menu);
                // ...but is also an mdc-menu-surface
                let surface = MDCMenuSurface::new(elem);
                surface.listen("MDCMenuSurface:closed", &self.close_callback);
                self.surface = Some(surface);
            }
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::Closed => {
                self.props.onclose.emit(());
            }
        }
        false
    }

    fn view(&self) -> Html {
        html! {
            <div class="mdc-menu mdc-menu-surface" id=self.props.id.clone()
                 ref=self.node_ref.clone()>
                <ul class="mdc-list" role="menu" aria-hidden="true" aria-orientation="vertical" tabindex="-1">
                    { self.props.children.clone() }
                </ul>
            </div>
        }
    }

    fn destroy(&mut self) {
        if let Some(surface) = &self.surface {
            surface.unlisten("MDCMenuSurface:closed", &self.close_callback);
            surface.destroy();
        }
        if let Some(inner) = &self.inner {
            inner.destroy();
        }
    }
}
