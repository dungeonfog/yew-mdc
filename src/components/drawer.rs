use boolinator::Boolinator;
use web_sys::Element;
use yew::prelude::*;

pub mod content;
pub use content::Content;
pub mod header;
pub use header::Header;

use crate::mdc_sys::MDCDrawer;

pub struct Drawer {
    node_ref: NodeRef,
    inner: Option<MDCDrawer>,
}

#[derive(Properties, Clone, PartialEq)]
pub struct Props {
    #[prop_or_default]
    pub id: String,
    pub children: Children,
    #[prop_or_default]
    pub dismissible: bool,
    #[prop_or_default]
    pub open: bool,
}

impl Component for Drawer {
    type Message = ();
    type Properties = Props;

    fn create(_ctx: &Context<Self>) -> Self {
        Self {
            node_ref: NodeRef::default(),
            inner: None,
        }
    }

    fn changed(&mut self, ctx: &Context<Self>) -> bool {
        if let Some(drawer) = &self.inner {
            drawer.set_open(ctx.props().open);
        }
        true
    }

    fn update(&mut self, _ctx: &Context<Self>, _msg: Self::Message) -> bool {
        false
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let classes = classes!(
            "mdc-drawer",
            ctx.props().dismissible.as_some("mdc-drawer--dismissible")
        );
        html! {
            <aside class={classes} id={ctx.props().id.clone()} ref={self.node_ref.clone()}>
                { ctx.props().children.clone() }
            </aside>
        }
    }

    fn rendered(&mut self, ctx: &Context<Self>, first_render: bool) {
        if first_render {
            if let Some(drawer) = self.inner.take() {
                drawer.destroy();
            }
            if let Some(elem) = self.node_ref.cast::<Element>() {
                let drawer = MDCDrawer::new(elem);
                drawer.set_open(ctx.props().open);
                self.inner = Some(drawer);
            }
        }
    }

    fn destroy(&mut self, _ctx: &Context<Self>) {
        if let Some(drawer) = &self.inner {
            drawer.destroy();
        }
    }
}
