use yew::prelude::*;

pub mod primary_action;
pub use primary_action::PrimaryAction;
pub mod media;
pub use media::Media;
pub use media::Style as MediaStyle;

pub struct Card {
    id: String,
    props: Props,
    link: ComponentLink<Self>,
}

#[derive(Properties, Clone)]
pub struct Props {
    pub id: Option<String>,
    pub children: Children,
    pub outlined: bool,
    pub classes: String,
    pub oncontextclick: Option<Callback<ContextMenuEvent>>,
    pub onhoverenter: Option<Callback<MouseEnterEvent>>,
    pub onhoverleave: Option<Callback<MouseLeaveEvent>>,
}

pub enum Msg {
    RightClick(ContextMenuEvent),
    HoverEnter(MouseEnterEvent),
    HoverLeave(MouseLeaveEvent),
}

impl Component for Card {
    type Message = Msg;
    type Properties = Props;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        let id = props
            .id
            .as_ref()
            .map(|s| s.to_owned())
            .unwrap_or_else(|| format!("card-{}", crate::next_id()));
        Self { id, props, link }
    }

    fn change(&mut self, props: Props) -> ShouldRender {
        self.props = props;
        true
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::RightClick(event) => {
                if let Some(callback) = &self.props.oncontextclick {
                    callback.emit(event);
                }
            }
            Msg::HoverEnter(event) => {
                if let Some(callback) = &self.props.onhoverenter {
                    callback.emit(event);
                }
            }
            Msg::HoverLeave(event) => {
                if let Some(callback) = &self.props.onhoverleave {
                    callback.emit(event);
                }
            }
        }
        false
    }

    fn view(&self) -> Html {
        let outlined = if self.props.outlined {
            "mdc-card--outlined"
        } else {
            ""
        };
        let classes = format!("mdc-card {} {}", self.props.classes, outlined);
        let emit_contextclick = self.link.callback(Msg::RightClick);
        let emit_hoverenter = self.link.callback(Msg::HoverEnter);
        let emit_hoverleave = self.link.callback(Msg::HoverLeave);
        html! {
            <div class=classes id=self.id
                 oncontextmenu=emit_contextclick
                 onmouseenter=emit_hoverenter
                 onmouseleave=emit_hoverleave>
                { self.props.children.render() }
            </div>
        }
    }
}
