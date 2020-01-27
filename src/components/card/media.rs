use yew::prelude::*;

pub struct Media {
    id: String,
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

#[derive(Properties, Clone)]
pub struct Props {
    pub id: Option<String>,
    pub children: Children,
    pub style: Style,
    pub classes: String,
    pub raw_css: String,
}

impl Component for Media {
    type Message = ();
    type Properties = Props;

    fn create(props: Self::Properties, _link: ComponentLink<Self>) -> Self {
        let id = props
            .id
            .as_ref()
            .map(|s| s.to_owned())
            .unwrap_or_else(|| format!("card-media-{}", crate::next_id()));
        Self { id, props }
    }

    fn change(&mut self, props: Props) -> ShouldRender {
        self.props = props;
        true
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
                <div id=self.id class=classes></div>
            }
        } else {
            html! {
                <div id=self.id class=classes style=&self.props.raw_css>
                    <div class="mdc-card__media-content">
                        { self.props.children.render() }
                    </div>
                </div>
            }
        }
    }
}
