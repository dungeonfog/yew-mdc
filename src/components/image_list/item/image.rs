use yew::prelude::*;

pub struct Image;

#[derive(Properties, Clone, PartialEq)]
pub struct Props {
    pub src: String,
    #[prop_or_default]
    pub id: String,
    #[prop_or_default]
    pub classes: String,
    #[prop_or_default]
    pub container_id: String,
    #[prop_or_default]
    pub container_classes: String,
}

use std::rc::Rc;
impl Image {
    // Note: This should probably be done for any component exposing "classes"
    // to improve performance by avoiding `format!` whereever unnecessary.
    // An experimental trait for this is trivial to implement; I need to look
    // into this more tho.
    fn container_classes(&self, props: &Props) -> Rc<String> {
        if props.container_classes.is_empty() {
            Rc::new("mdc-image-list__image-aspect-container".to_string())
        } else {
            Rc::new(format!(
                "mdc-image-list__image-aspect-container {}",
                props.container_classes
            ))
        }
    }
    fn classes(&self, props: &Props) -> Rc<String> {
        if props.classes.is_empty() {
            Rc::new("mdc-image-list__image".to_string())
        } else {
            Rc::new(format!("mdc-image-list__image {}", props.classes))
        }
    }
}

impl Component for Image {
    type Message = ();
    type Properties = Props;

    fn create(_ctx: &Context<Self>) -> Self {
        Self
    }

    fn changed(&mut self, _ctx: &Context<Self>) -> bool {
        true
    }

    fn update(&mut self, _ctx: &Context<Self>, _msg: Self::Message) -> bool {
        false
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        html! {
            <div id={ctx.props().container_id.clone()}
                 class={self.container_classes(ctx.props()).as_ref()}
                >
                <img id={ctx.props().id.clone()}
                     src={ctx.props().src.clone()}
                     class={self.classes(ctx.props()).as_ref()}
                    />
            </div>
        }
    }
}
