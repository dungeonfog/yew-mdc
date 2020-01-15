use yew::prelude::*;

pub struct Content {
    props: Props,
}

#[derive(Properties, Clone)]
pub struct Props {
    children: Children,
}

impl Component for Content {
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
            <div class="mdc-dialog__content">
                { self.props.children.render() }
            </div>
        }
    }
}
