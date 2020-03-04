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

#[derive(Properties, Clone, PartialEq)]
pub struct Props {
    pub children: Children,
    #[prop_or_default]
    pub id: Option<String>,
    #[prop_or_default]
    pub outlined: bool,
    #[prop_or_default]
    pub classes: String,
    #[prop_or_default]
    pub raw_css: String,
    #[prop_or_else(Callback::noop)]
    pub oncontextclick: Callback<MouseEvent>,
    #[prop_or_else(Callback::noop)]
    pub onhoverenter: Callback<MouseEvent>,
    #[prop_or_else(Callback::noop)]
    pub onhoverleave: Callback<MouseEvent>,
}

pub enum Msg {
    RightClick(MouseEvent),
    HoverEnter(MouseEvent),
    HoverLeave(MouseEvent),
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
        if self.props != props {
            self.props = props;
            true
        } else {
            false
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::RightClick(event) => {
                if self.props.oncontextclick != Callback::noop() {
                    event.prevent_default();
                    self.props.oncontextclick.emit(event);
                }
            }
            Msg::HoverEnter(event) => {
                self.props.onhoverenter.emit(event);
            }
            Msg::HoverLeave(event) => {
                self.props.onhoverleave.emit(event);
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
                 style=&self.props.raw_css
                 oncontextmenu=emit_contextclick
                 onmouseenter=emit_hoverenter
                 onmouseleave=emit_hoverleave>
                { self.props.children.render() }
            </div>
        }
    }
}
