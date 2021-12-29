use yew::prelude::*;

pub mod section;
pub use section::Section;

pub struct TopAppBar {
    props: Props,
}

#[derive(Properties, Clone, PartialEq)]
pub struct Props {
    pub children: Children,
    #[prop_or_default]
    pub id: String,
    #[prop_or_default]
    pub manualrows: bool,
    #[prop_or_default]
    pub classes: String,
}

impl Component for TopAppBar {
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
        let classes = format!("mdc-top-app-bar {}", self.props.classes);
        if self.props.manualrows {
            html! {
                <header class=classes id=self.props.id.clone()>
                    { self.props.children.clone() }
                </header>
            }
        } else {
            html! {
                <header class=classes id=self.props.id.clone()>
                    <div class="mdc-top-app-bar__row">
                        { self.props.children.clone() }
                    </div>
                </header>
            }
        }
    }

    // TODO: Wrap related JavaScript
    // TODO: Do we need it?
    //fn destroy(&mut self) {}
}
