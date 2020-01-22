use crate::mdc_sys::MDCRipple;
use yew::prelude::*;

pub struct PrimaryAction {
    id: String,
    props: Props,
    ripple: Option<MDCRipple>,
}

#[derive(Properties, Clone)]
pub struct Props {
    pub id: Option<String>,
    pub children: Children,
}

impl Component for PrimaryAction {
    type Message = ();
    type Properties = Props;

    fn create(props: Self::Properties, _link: ComponentLink<Self>) -> Self {
        let id = props
            .id
            .as_ref()
            .map(|s| s.to_owned())
            .unwrap_or_else(|| format!("card-primary-action-{}", crate::next_id()));
        Self {
            id,
            props,
            ripple: None,
        }
    }

    fn mounted(&mut self) -> ShouldRender {
        self.ripple = crate::get_element_by_id(&self.id).map(MDCRipple::new);
        false
    }

    fn change(&mut self, props: Props) -> ShouldRender {
        if self.props.id != props.id {
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
        html! {
            <div id=self.id class="mdc-card__primary-action" tabindex="0">
                { self.props.children.render() }
            </div>
        }
    }

    fn destroy(&mut self) {
        if let Some(ripple) = &self.ripple {
            ripple.destroy();
        }
    }
}
