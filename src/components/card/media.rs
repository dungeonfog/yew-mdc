use yew::prelude::*;

pub struct Media {
    props: Props,
}

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

    fn create(props: Self::Properties, _link: ComponentLink<Self>) -> Self {
        Self { props }
    }

    fn change(&mut self, props: Props) -> ShouldRender {
        if self.props != props {
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
        let classes = format!(
            "mdc-card__media {} {}",
            self.props.style, self.props.classes
        );
        if self.props.children.is_empty() {
            html! {
                <div id=self.props.id.clone() class=classes></div>
            }
        } else {
            html! {
                <div id=self.props.id.clone() class=classes style=self.props.raw_css.clone()>
                    <div class="mdc-card__media-content">
                        { self.props.children.clone() }
                    </div>
                </div>
            }
        }
    }
}
