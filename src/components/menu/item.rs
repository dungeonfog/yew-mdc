use yew::prelude::*;

pub struct Item {
    id: String,
    props: Props,
}

#[derive(Properties)]
pub struct Props {
    pub id: Option<String>,
    pub text: String,
    pub disabled: bool,
    pub(crate) onclick: Option<Callback<()>>,
}

pub enum Msg {
    Clicked,
}

impl Component for Item {
    type Message = Msg;
    type Properties = Props;

    fn create(props: Self::Properties, _link: ComponentLink<Self>) -> Self {
        let id = props
            .id
            .as_ref()
            .map(|s| s.to_owned())
            .unwrap_or_else(|| format!("text-field-{}", crate::next_id()));
        Self { id, props }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::Clicked => {
                if let Some(callback) = &self.props.onclick {
                    callback.emit(());
                }
            }
        }
        false
    }

    fn view(&self) -> Html<Self> {
        let classes = if self.props.disabled {
            "mdc-list-item mdc-list-item--disabled"
        } else {
            "mdc-list-item"
        };
        html! {
            <li class=classes role="menuitem" id=self.id onclick=|_| Msg::Clicked>
                <span class="mdc-list-item__text">{ &self.props.text }</span>
            </li>
        }
    }
}
