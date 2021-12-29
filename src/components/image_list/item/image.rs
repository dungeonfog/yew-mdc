use yew::prelude::*;

pub struct Image {
    props: Props,
}

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
    fn container_classes(&self) -> Rc<String> {
        if self.props.container_classes.is_empty() {
            Rc::new("mdc-image-list__image-aspect-container".to_string())
        } else {
            Rc::new(format!(
                "mdc-image-list__image-aspect-container {}",
                self.props.container_classes
            ))
        }
    }
    fn classes(&self) -> Rc<String> {
        if self.props.classes.is_empty() {
            Rc::new("mdc-image-list__image".to_string())
        } else {
            Rc::new(format!("mdc-image-list__image {}", self.props.classes))
        }
    }
}

impl Component for Image {
    type Message = ();
    type Properties = Props;

    fn create(props: Self::Properties, _link: ComponentLink<Self>) -> Self {
        Self { props }
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        if self.props != props {
            self.props = props;
            true
        } else {
            false
        }
    }

    fn update(&mut self, _msg: Self::Message) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        html! {
            <div id=self.props.container_id.clone()
                 class=self.container_classes().as_ref()
                >
                <img id=self.props.id.clone()
                     src=self.props.src.clone()
                     class=self.classes().as_ref()
                    />
            </div>
        }
    }
}
