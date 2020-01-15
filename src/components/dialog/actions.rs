use yew::prelude::*;

pub struct Actions {
    props: Props,
}

#[derive(Properties, Clone)]
pub struct Props {
    children: Children,
}

impl Component for Actions {
    type Message = ();
    type Properties = Props;

    fn create(props: Props, _link: ComponentLink<Self>) -> Self {
        Self { props }
    }
    fn change(&mut self, props: Props) -> ShouldRender {
        self.props = props;
        true
    }
    fn update(&mut self, _msg: Self::Message) -> ShouldRender {
        false
    }
    fn view(&self) -> Html {
        html! {
            <div class="mdc-dialog__actions">
                { self.props.children.render() }
            </div>
        }
    }
}
