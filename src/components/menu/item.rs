use yew::prelude::*;

pub struct Item {
    props: Props,
    link: ComponentLink<Self>,
}

#[derive(Properties, Clone, PartialEq)]
pub struct Props {
    #[prop_or_default]
    pub id: String,
    #[prop_or_default]
    pub text: String,
    #[prop_or_default]
    pub disabled: bool,
    #[prop_or_else(Callback::noop)]
    pub onclick: Callback<MouseEvent>,
    #[prop_or_default]
    pub children: Children,
}

pub enum Msg {
    Clicked(MouseEvent),
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
        match msg {
            Msg::Clicked(ev) => {
                if !self.props.disabled {
                    self.props.onclick.emit(ev);
                }
            }
        }
        false
    }

    fn view(&self) -> Html {
        let classes = if self.props.disabled {
            "mdc-list-item mdc-list-item--disabled"
        } else {
            "mdc-list-item"
        };
        let onclick = self.link.callback(Msg::Clicked);
        let text = if self.props.text.is_empty() {
            html! {}
        } else {
            html! {
                <span class="mdc-list-item__text">{ &self.props.text }</span>
            }
        };
        html! {
            <li class=classes role="menuitem" id=self.props.id.clone() onclick=onclick>
                { self.props.children.clone() }
                { text }
            </li>
        }
    }
}
