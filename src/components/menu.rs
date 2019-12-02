use crate::mdc_sys::{MDCMenu, MDCMenuSurface};
use wasm_bindgen::prelude::*;
use yew::prelude::*;

pub mod item;
pub use item::Item;

pub struct Menu {
    id: String,
    inner: Option<MDCMenu>,
    surface: Option<MDCMenuSurface>,
    close_callback: Closure<dyn FnMut(web_sys::Event)>,
    props: Props,
}

#[derive(Properties)]
pub struct Props {
    pub id: Option<String>,
    pub onitemclicked: Option<Callback<usize>>,
    pub onclose: Option<Callback<()>>,
    pub fixed_position: bool,
    pub absolute_position: Option<(u32, u32)>,
    pub items: Vec<item::Props>,
    pub open: bool,
}

pub enum Msg {
    ItemClicked(usize),
    Closed,
}

impl Component for Menu {
    type Message = Msg;
    type Properties = Props;

    fn create(props: Self::Properties, mut link: ComponentLink<Self>) -> Self {
        let id = props
            .id
            .as_ref()
            .map(|s| s.to_owned())
            .unwrap_or_else(|| format!("menu-{}", crate::next_id()));
        let callback = link.send_back(|_: ()| Msg::Closed);
        let closure = Closure::wrap(Box::new(move |e: web_sys::Event| {
            e.stop_propagation();
            callback.emit(());
        }) as Box<dyn FnMut(web_sys::Event)>);
        Self {
            id,
            inner: None,
            surface: None,
            close_callback: closure,
            props,
        }
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        if props.open != self.props.open {
            self.props.open = props.open;
            if let Some(inner) = &self.inner {
                inner.set_open(props.open);
            }
        }
        false
    }

    fn mounted(&mut self) -> ShouldRender {
        if let Some(elem) = crate::get_element_by_id(&self.id) {
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
        false
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::ItemClicked(index) => {
                if let Some(callback) = &self.props.onitemclicked {
                    callback.emit(index);
                }
            }
            Msg::Closed => {
                if let Some(callback) = &self.props.onclose {
                    callback.emit(());
                }
            }
        }
        false
    }

    fn view(&self) -> Html<Self> {
        let children = html! {{
            for self.props.items.iter().enumerate().map(|(index, item)| {
                html! {
                    <Item id=&item.id text=&item.text onclick=move |_| Msg::ItemClicked(index) />
                }
            })
        }};
        html! {
            <div class="mdc-menu mdc-menu-surface" id=self.id>
                <ul class="mdc-list" role="menu" aria-hidden="true" aria-orientation="vertical" tabindex="-1">
                    { children }
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
