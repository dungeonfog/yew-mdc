use boolinator::Boolinator;
use yew::prelude::*;

pub struct Item {
    props: Props,
}

#[derive(Properties, Clone, PartialEq)]
pub struct Props {
    #[prop_or_default]
    pub id: String,
    #[prop_or_default]
    pub text: String,
    #[prop_or_default]
    pub value: String,
    #[prop_or_default]
    pub disabled: bool,
    #[prop_or_default]
    pub children: Children,
    #[prop_or_else(Callback::noop)]
    pub onclick: Callback<MouseEvent>,
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
        let classes = classes![
            "mdc-deprecated-list-item",
            self.props
                .disabled
                .as_some("mdc-deprecated-list-item--disabled")
        ];
        let text = if self.props.text.is_empty() {
            html! {}
        } else {
            html! {
                <span class="mdc-deprecated-list-item__text">{ &self.props.text }</span>
            }
        };
        html! {
            <li class=classes
                role="option"
                id=self.props.id.clone()
                data-value=self.props.value.clone()>

                { self.props.children.clone() }
                <span class="mdc-deprecated-list-item__ripple"></span>
                { text }
            </li>
        }
    }
}
