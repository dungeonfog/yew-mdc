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
}

pub enum Msg {
    RightClick(ContextMenuEvent),
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
        if self.props.id != props.id
            || self.props.outlined != props.outlined
            || self.props.classes != props.classes
        {
            self.props = props;
            true
        } else {
            false
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::RightClick(event) => {
                if let Some(callback) = &self.props.oncontextclick {
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
        html! {
            <div class=classes id=self.id oncontextmenu=emit_contextclick>
                { self.props.children.render() }
            </div>
        }
    }
}
