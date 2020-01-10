use yew::prelude::*;

pub struct HelperLine {
    props: Props,
}

#[derive(Properties, Clone)]
pub struct Props {
    pub children: Children,
    pub persistent: bool,
    pub validation_msg: bool,
}

impl Component for HelperLine {
    type Message = ();
    type Properties = Props;

    fn create(props: Self::Properties, _link: ComponentLink<Self>) -> Self {
        Self { props }
    }

    fn update(&mut self, _msg: Self::Message) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        let persistent = if self.props.persistent {
            " mdc-text-field-helper-text--persistent"
        } else {
            ""
        };
        let validation_msg = if self.props.validation_msg {
            " mdc-text-field-helper-text--validation-msg"
        } else {
            ""
        };
        let classes = format!("mdc-text-field-helper-text{}{}", persistent, validation_msg);
        html! {
            <div class="mdc-text-field-helper-line">
                <div class=classes aria-hidden="true">
                    { self.props.children.render() }
                </div>
            </div>
        }
    }
}
