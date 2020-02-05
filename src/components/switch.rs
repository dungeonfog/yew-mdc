use crate::mdc_sys::MDCSwitch;
use yew::prelude::*;

// Note: This name conflicts with yew_router::switch::Switch.
// Not sure what to think about that.
pub struct Switch {
    id: String,
    id_inner: String,
    link: ComponentLink<Self>,
    inner: Option<MDCSwitch>,
    props: Props,
    state: bool,
}

#[derive(Properties, Clone)]
pub struct Props {
    pub id: Option<String>,
    pub state: bool,
    pub onchange: Option<Callback<bool>>,
    pub label_text: String,
}

pub enum Msg {
    StateChanged,
}

impl Component for Switch {
    type Message = Msg;
    type Properties = Props;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        let id = props
            .id
            .as_ref()
            .map(|s| s.to_owned())
            .unwrap_or_else(|| format!("switch-{}", crate::next_id()));
        let id_inner = format!("{}-inner", id);
        Self {
            id,
            id_inner,
            link,
            inner: None,
            state: props.state,
            props,
        }
    }

    fn mounted(&mut self) -> ShouldRender {
        self.inner = crate::get_element_by_id(&self.id).map(MDCSwitch::new);
        false
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::StateChanged => {
                self.state = !self.state;
                if let Some(ref callback) = self.props.onchange {
                    callback.emit(self.state);
                }
                true
            }
        }
    }

    fn view(&self) -> Html {
        let emit_change = self.link.callback(|_| Msg::StateChanged);
        let (on_class, checked) = if self.state {
            (" mdc-switch--checked", true)
        } else {
            ("", false)
        };
        let classes = format!("mdc-switch{}", on_class);
        let switch = html! {
            <div id=&self.id class=classes>
                <div class="mdc-switch__track"></div>
                <div class="mdc-switch__thumb-underlay">
                    <div class="mdc-switch__thumb">
                        <input type="checkbox"
                               id=&self.id_inner
                               onchange=emit_change
                               class="mdc-switch__native-control"
                               checked=checked
                            />
                    </div>
                </div>
            </div>
        };
        if self.props.label_text.is_empty() {
            switch
        } else {
            html! {
                <>
                { switch }
                <label for=&self.id_inner>{ &self.props.label_text }</label>
                </>
            }
        }
    }

    fn destroy(&mut self) {
        if let Some(ref inner) = self.inner {
            inner.destroy();
        }
    }
}
