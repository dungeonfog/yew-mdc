use yew::prelude::*;

pub mod content;
pub use content::Content;
pub mod header;
pub use header::Header;

pub struct Drawer {
    id: String,
    // inner: Option<MDCDrawer>,
    props: Props,
}

#[derive(Properties, Clone, PartialEq)]
pub struct Props {
    pub id: Option<String>,
    pub children: Children,
}

impl Component for Drawer {
    type Message = ();
    type Properties = Props;

    fn create(props: Self::Properties, _link: ComponentLink<Self>) -> Self {
        let id = props
            .id
            .as_ref()
            .map(|s| s.to_owned())
            .unwrap_or_else(|| format!("drawer-{}", crate::next_id()));
        Self { id, props }
    }

    fn change(&mut self, props: Props) -> ShouldRender {
        if self.props != props {
            self.props = props;
            true
        } else {
            false
        }
    }

    // fn mounted(&mut self) -> ShouldRender {
    //     false
    // }

    fn update(&mut self, _msg: Self::Message) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        html! {
            <aside class="mdc-drawer" id=self.id>
                { self.props.children.render() }
            </aside>
        }
    }

    // fn destroy(&mut self) {}
}
