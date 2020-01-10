use yew::prelude::*;

pub struct Header {
    id: String,
    props: Props,
}

#[derive(Properties, Clone)]
pub struct Props {
    pub id: Option<String>,
    pub children: Children,
}

impl Component for Header {
    type Message = ();
    type Properties = Props;

    fn create(props: Self::Properties, _link: ComponentLink<Self>) -> Self {
        let id = props
            .id
            .as_ref()
            .map(|s| s.to_owned())
            .unwrap_or_else(|| format!("drawer-header-{}", crate::next_id()));
        Self { id, props }
    }

    fn update(&mut self, _msg: Self::Message) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        html! {
            <div class="mdc-drawer__header" id=self.id>
                { self.props.children.render() }
            </div>
        }
    }
}
