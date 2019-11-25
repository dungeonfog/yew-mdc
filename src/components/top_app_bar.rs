use yew::prelude::*;

pub mod section;
pub use section::Section;

pub struct TopAppBar {
    id: String,
    props: Props,
}

#[derive(Properties)]
pub struct Props {
    pub id: Option<String>,
    pub children: Children<TopAppBar>,
    pub manualrows: bool,
    pub classes: String,
}

impl Component for TopAppBar {
    type Message = ();
    type Properties = Props;

    fn create(props: Self::Properties, _link: ComponentLink<Self>) -> Self {
        let id = props
            .id
            .as_ref()
            .map(|s| s.to_owned())
            .unwrap_or_else(|| format!("top-app-bar-{}", crate::next_id()));
        Self { id, props }
    }

    // TODO: Wrap related JavaScript
    //fn mounted(&mut self) -> ShouldRender {
    //    false
    //}

    fn update(&mut self, _msg: Self::Message) -> ShouldRender {
        false
    }

    fn view(&self) -> Html<Self> {
        let classes = format!("mdc-top-app-bar {}", self.props.classes);
        if self.props.manualrows {
            html! {
                <header class=classes id=self.id>
                    { self.props.children.render() }
                </header>
            }
        } else {
            html! {
                <header class=classes id=self.id>
                    <div class="mdc-top-app-bar__row">
                        { self.props.children.render() }
                    </div>
                </header>
            }
        }
    }

    // TODO: Wrap related JavaScript
    //fn destroy(&mut self) {}
}
