use crate::mdc_sys::MDCMenu;
use yew::prelude::*;

pub struct Menu {
    id: String,
    inner: Option<MDCMenu>,
    props: Props,
}

#[derive(Properties)]
pub struct Props {
    pub id: Option<String>,
    pub children: Children<Menu>,
    pub open: bool,
}

impl Component for Menu {
    type Message = ();
    type Properties = Props;

    fn create(props: Self::Properties, _link: ComponentLink<Self>) -> Self {
        let id = props
            .id
            .as_ref()
            .map(|s| s.to_owned())
            .unwrap_or_else(|| format!("menu-{}", crate::next_id()));
        Self {
            id,
            inner: None,
            props,
        }
    }

    fn mounted(&mut self) -> ShouldRender {
        if let Some(menu) = crate::get_element_by_id(&self.id).map(MDCMenu::new) {
            if self.props.open {
                menu.set_open(true);
            }
            self.inner = Some(menu);
        }
        false
    }

    fn update(&mut self, _msg: Self::Message) -> ShouldRender {
        false
    }

    fn view(&self) -> Html<Self> {
        html! {
            <div class="mdc-menu mdc-menu-surface" id=self.id>
                <ul class="mdc-list" role="menu" aria-hidden="true" aria-orientation="vertical" tabindex="-1">
                    { self.props.children.render() }
                </ul>
            </div>
        }
    }

    fn destroy(&mut self) {
        if let Some(inner) = &self.inner {
            inner.destroy();
        }
    }
}
