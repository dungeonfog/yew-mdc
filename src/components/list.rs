use web_sys::Element;
use yew::prelude::*;

use crate::mdc_sys::MDCList;

pub mod item;
pub use item::Item;

pub struct List {
    node_ref: NodeRef,
    inner: Option<MDCList>,
}

#[derive(Properties, Clone, PartialEq)]
pub struct Props {
    pub children: Children,
    #[prop_or_default]
    pub id: String,
    #[prop_or_default]
    pub classes: String,
}

impl Component for List {
    type Message = ();
    type Properties = Props;

    fn create(_ctx: &Context<Self>) -> Self {
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
            self.inner = self.node_ref.cast::<Element>().map(MDCList::new);
        }
    }

    fn changed(&mut self, _ctx: &Context<Self>) -> bool {
        true
    }

    fn update(&mut self, _ctx: &Context<Self>, _msg: Self::Message) -> bool {
        false
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        html! {
            <ul id={ctx.props().id.clone()} class={format!("mdc-list {}", ctx.props().classes)} ref={self.node_ref.clone()}>
                { ctx.props().children.clone() }
            </ul>
        }
    }

    fn destroy(&mut self, _ctx: &Context<Self>) {
        if let Some(inner) = &self.inner {
            inner.destroy();
        }
    }
}
