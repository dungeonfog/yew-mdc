use std::fmt;
use yew::prelude::*;

pub struct Section {
    id: String,
    props: Props,
}

#[derive(PartialEq, Clone, Copy, Debug)]
pub enum Align {
    Start,
    End,
}

impl Default for Align {
    fn default() -> Align {
        Align::Start
    }
}
impl fmt::Display for Align {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let align = match self {
            Align::Start => "start",
            Align::End => "end",
        };
        write!(f, "mdc-top-app-bar__section--align-{}", align)
    }
}

#[derive(Properties, Clone, PartialEq)]
pub struct Props {
    pub children: Children,
    #[prop_or_default]
    pub id: Option<String>,
    #[prop_or_default]
    pub align: Align,
}

impl Component for Section {
    type Message = ();
    type Properties = Props;

    fn create(props: Self::Properties, _link: ComponentLink<Self>) -> Self {
        let id = props
            .id
            .as_ref()
            .map(|s| s.to_owned())
            .unwrap_or_else(|| format!("app-bar-section-{}", crate::next_id()));
        Self { id, props }
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
        let classes = format!("mdc-top-app-bar__section {}", self.props.align);
        html! {
            <section class=classes id=self.id>
                { self.props.children.render() }
            </section>
        }
    }
}
