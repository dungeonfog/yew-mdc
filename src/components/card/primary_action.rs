use crate::mdc_sys::MDCRipple;
use yew::prelude::*;

pub struct PrimaryAction {
    id: String,
    props: Props,
    ripple: Option<MDCRipple>,
    link: ComponentLink<Self>,
}

#[derive(Properties, Clone)]
pub struct Props {
    pub id: Option<String>,
    pub children: Children,
    pub onclick: Option<Callback<ClickEvent>>,
    pub oncontextclick: Option<Callback<ContextMenuEvent>>,
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
        self.props = props;
        true
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::LeftClick(event) => {
                if let Some(callback) = &self.props.onclick {
                    callback.emit(event);
                }
            }
            Msg::RightClick(event) => {
                if let Some(callback) = &self.props.oncontextclick {
                    callback.emit(event);
                }
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
