use crate::mdc_sys::MDCDrawer;
use web_sys::Element;
use yew::prelude::*;

pub mod content;
pub use content::Content;
pub mod header;
pub use header::Header;

pub struct Drawer {
    node_ref: NodeRef,
    inner: Option<MDCDrawer>,
    props: Props,
}

#[derive(PartialEq, Clone, Copy, Debug)]
pub enum Style {
    None,
    Dismissible,
    Modal,
}
impl Default for Style {
    fn default() -> Style {
        Style::None
    }
}
impl std::fmt::Display for Style {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        use Style::*;
        let result = match self {
            None => "",
            Dismissible => "mdc-drawer--dismissible",
            Modal => "mdc-drawer--modal",
        };
        write!(f, "{}", result)
    }
}

#[derive(Properties, Clone, PartialEq)]
pub struct Props {
    #[prop_or_default]
    pub id: String,
    pub children: Children,
    #[prop_or_default]
    pub style: Style,
    #[prop_or_default]
    pub classes: String,
    #[prop_or_default]
    pub open: bool,
}

impl Component for Drawer {
    type Message = ();
    type Properties = Props;

    fn create(props: Self::Properties, _link: ComponentLink<Self>) -> Self {
        Self {
            node_ref: NodeRef::default(),
            props,
            inner: None,
        }
    }

    fn rendered(&mut self, first_render: bool) {
        if first_render {
            if let Some(inner) = self.inner.take() {
                inner.destroy()
            }
            if let Some(elem) = self.node_ref.cast::<Element>() {
                let drawer = MDCDrawer::new(elem.clone());
                drawer.set_open(self.props.open);
                self.inner = Some(drawer)
            }
        }
    }

    fn change(&mut self, props: Props) -> ShouldRender {
        if let Some(inner) = &self.inner {
            if props.open != self.props.open {
                self.props.open = props.open;
                inner.set_open(props.open);
            }
        }
        if self.props != props {
            self.props = props;
            true
        } else {
            false
        }
    }

    // fn mounted(&mut self) -> ShouldRender {
    //     false
    // }

    fn update(&mut self, _msg: Self::Message) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        html! {
            <>
            <aside
                class=format!("mdc-drawer {} {}", self.props.style, self.props.classes)
                id=self.props.id
                ref=self.node_ref.clone()
            >
                { self.props.children.clone() }
            </aside>
            {if self.props.style == Style::Modal {html!{<div class="mdc-drawer-scrim"></div>}} else {html!{}}}
            </>
        }
    }

    fn destroy(&mut self) {
        if let Some(inner) = &self.inner {
            inner.destroy()
        }
    }
}
