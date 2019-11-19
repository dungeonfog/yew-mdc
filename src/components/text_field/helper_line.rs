use yew::prelude::*;

pub struct HelperLine {
    props: Props,
}

#[derive(Properties)]
pub struct Props {
    children: Children<HelperLine>,
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

    fn view(&self) -> Html<Self> {
        html! {
            <div class="mdc-text-field-helper-line">
                <div class="mdc-text-field-helper-text" aria-hidden="true">
                    { self.props.children.render() }
                </div>
            </div>
        }
    }
}
