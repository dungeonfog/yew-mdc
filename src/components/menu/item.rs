use yew::prelude::*;

pub struct Item {
    id: String,
    props: Props,
    link: ComponentLink<Self>,
}

#[derive(Properties, Clone, PartialEq)]
pub struct Props {
    pub id: Option<String>,
    pub text: String,
    pub disabled: bool,
    pub onclick: Option<Callback<()>>,
    pub children: Children,
}

pub enum Msg {
    Clicked,
}

impl Component for Item {
    type Message = Msg;
    type Properties = Props;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        let id = props
            .id
            .as_ref()
            .map(|s| s.to_owned())
            .unwrap_or_else(|| format!("menu-item-{}", crate::next_id()));
        Self { id, props, link }
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
            Msg::Clicked => {
                if !self.props.disabled {
                    if let Some(callback) = &self.props.onclick {
                        callback.emit(());
                    }
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
        let onclick = self.link.callback(|_| Msg::Clicked);
        html! {
            <li class=classes role="menuitem" id=self.id onclick=onclick>
                { self.props.children.render() }
                <span class="mdc-list-item__text">{ &self.props.text }</span>
            </li>
        }
    }
}
