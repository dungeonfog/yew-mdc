use yew::prelude::*;

pub mod image;
pub use image::Image;
pub mod supporting;
pub use supporting::Supporting;

pub struct Item {
    props: Props,
}

#[derive(Properties, Clone, PartialEq)]
pub struct Props {
    pub children: Children,
    #[prop_or_default]
    pub id: String,
    #[prop_or_default]
    pub classes: String,
}

impl Component for Item {
    type Message = ();
    type Properties = Props;

    fn create(props: Self::Properties, _link: ComponentLink<Self>) -> Self {
        Self { props }
    }

    fn change(&mut self, props: Props) -> ShouldRender {
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
            <li id=&self.props.id class=format!("mdc-image-list__item {}", self.props.classes)>
                { self.props.children.render() }
            </li>
        }
    }
}
