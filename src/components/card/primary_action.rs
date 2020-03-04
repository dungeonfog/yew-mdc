use crate::mdc_sys::MDCRipple;
use stdweb::traits::IEvent;
use yew::prelude::*;

pub struct PrimaryAction {
    id: String,
    props: Props,
    ripple: Option<MDCRipple>,
    link: ComponentLink<Self>,
}

#[derive(Properties, Clone, PartialEq)]
pub struct Props {
    pub children: Children,
    #[prop_or_default]
    pub id: Option<String>,
    #[prop_or_else(Callback::noop)]
    pub onclick: Callback<ClickEvent>,
    #[prop_or_else(Callback::noop)]
    pub oncontextclick: Callback<ContextMenuEvent>,
}

pub enum Msg {
    LeftClick(ClickEvent),
    RightClick(ContextMenuEvent),
}

impl Component for PrimaryAction {
    type Message = Msg;
    type Properties = Props;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        let id = props
            .id
            .as_ref()
            .map(|s| s.to_owned())
            .unwrap_or_else(|| format!("card-primary-action-{}", crate::next_id()));
        Self {
            id,
            props,
            ripple: None,
            link,
        }
    }

    fn mounted(&mut self) -> ShouldRender {
        self.ripple = crate::get_element_by_id(&self.id).map(MDCRipple::new);
        false
    }

    fn change(&mut self, props: Props) -> ShouldRender {
        if self.props != props {
            self.props = props;
            true
        } else {
            false
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::LeftClick(event) => {
                self.props.onclick.emit(event);
            }
            Msg::RightClick(event) => {
                if self.props.oncontextclick != Callback::noop() {
                    event.prevent_default();
                }
                self.props.oncontextclick.emit(event);
            }
        }
        false
    }

    fn view(&self) -> Html {
        let emit_click = self.link.callback(Msg::LeftClick);
        let emit_contextclick = self.link.callback(Msg::RightClick);
        html! {
            <div
                id=self.id
                class="mdc-card__primary-action"
                tabindex="0"
                onclick=emit_click
                oncontextmenu=emit_contextclick>
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
