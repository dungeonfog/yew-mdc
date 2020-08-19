use yew::prelude::*;

pub mod image;
pub use image::Image;
pub mod supporting;
pub use supporting::Supporting;

pub struct Item {
    props: Props,
    link: ComponentLink<Self>,
}

#[derive(Properties, Clone, PartialEq)]
pub struct Props {
    pub children: Children,
    #[prop_or_default]
    pub id: String,
    #[prop_or_default]
    pub classes: String,
    #[prop_or_else(Callback::noop)]
    pub onclick: Callback<MouseEvent>,
    #[prop_or_else(Callback::noop)]
    pub onmouseenter: Callback<MouseEvent>,
    #[prop_or_else(Callback::noop)]
    pub onmouseleave: Callback<MouseEvent>,
}

pub enum Msg {
    Clicked(MouseEvent),
    MouseEnter(MouseEvent),
    MouseLeave(MouseEvent),
}

impl Component for Item {
    type Message = Msg;
    type Properties = Props;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self { props, link }
    }

    fn change(&mut self, props: Props) -> ShouldRender {
        if self.props != props {
            self.props = props;
            true
        } else {
            false
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        let (callback, event) = match msg {
            Msg::Clicked(ev) => (&self.props.onclick, ev),
            Msg::MouseEnter(ev) => (&self.props.onmouseenter, ev),
            Msg::MouseLeave(ev) => (&self.props.onmouseleave, ev),
        };
        callback.emit(event);
        false
    }

    fn view(&self) -> Html {
        html! {
            <li id=&self.props.id
                class=format!("mdc-image-list__item {}", self.props.classes)
                onclick=self.link.callback(Msg::Clicked)
                onmouseenter=self.link.callback(Msg::MouseEnter)
                onmouseleave=self.link.callback(Msg::MouseLeave)
                >
                { self.props.children.clone() }
            </li>
        }
    }
}
