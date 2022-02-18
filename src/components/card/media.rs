use yew::prelude::*;

pub struct Media;

#[derive(Clone, Copy, PartialEq)]
pub enum Style {
    None,
    Square,
    R16by9,
}
impl Default for Style {
    fn default() -> Style {
        Style::None
    }
}
impl std::fmt::Display for Style {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        use Style::*;
        let result = match self {
            None => "",
            Square => "mdc-card__media--square",
            R16by9 => "mdc-card__media--16-9",
        };
        write!(f, "{}", result)
    }
}

#[derive(Properties, Clone, PartialEq)]
pub struct Props {
    pub children: Children,
    #[prop_or_default]
    pub id: String,
    #[prop_or_default]
    pub style: Style,
    #[prop_or_default]
    pub classes: String,
    #[prop_or_default]
    pub raw_css: String,
}

impl Component for Media {
    type Message = ();
    type Properties = Props;

    fn create(_ctx: &Context<Self>) -> Self {
        Self
    }

    fn changed(&mut self, _ctx: &Context<Self>) -> bool {
        true
    }

    fn update(&mut self, _ctx: &Context<Self>, _msg: Self::Message) -> bool {
        false
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let props = ctx.props();
        let classes = format!("mdc-card__media {} {}", props.style, props.classes);
        if props.children.is_empty() {
            html! {
                <div id={props.id.clone()} class={classes}></div>
            }
        } else {
            html! {
                <div id={props.id.clone()} class={classes} style={props.raw_css.clone()}>
                    <div class="mdc-card__media-content">
                        { props.children.clone() }
                    </div>
                </div>
            }
        }
    }
}
