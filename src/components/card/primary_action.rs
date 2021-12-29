use crate::mdc_sys::MDCRipple;
use web_sys::Element;
use yew::prelude::*;

pub struct PrimaryAction {
    node_ref: NodeRef,
    props: Props,
    ripple: Option<MDCRipple>,
    link: ComponentLink<Self>,
}

#[derive(Properties, Clone, PartialEq)]
pub struct Props {
    pub children: Children,
    #[prop_or_default]
    pub id: String,
    #[prop_or_else(Callback::noop)]
    pub onclick: Callback<MouseEvent>,
    #[prop_or_else(Callback::noop)]
    pub oncontextclick: Callback<MouseEvent>,
}

pub enum Msg {
    LeftClick(MouseEvent),
    RightClick(MouseEvent),
}

impl Component for PrimaryAction {
    type Message = Msg;
    type Properties = Props;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self {
            node_ref: NodeRef::default(),
            props,
            ripple: None,
            link,
        }
    }

    fn rendered(&mut self, first_render: bool) {
        if first_render {
            if let Some(ripple) = self.ripple.take() {
                ripple.destroy();
            }
            self.ripple = self.node_ref.cast::<Element>().map(MDCRipple::new);
        }
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
                id=self.props.id.clone()
                ref=self.node_ref.clone()
                class="mdc-card__primary-action"
                tabindex="0"
                onclick=emit_click
                oncontextmenu=emit_contextclick>
                { self.props.children.clone() }
            </div>
        }
    }

    fn destroy(&mut self) {
        if let Some(ripple) = &self.ripple {
            ripple.destroy();
        }
    }
}
