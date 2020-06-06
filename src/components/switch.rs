use crate::mdc_sys::MDCSwitch;
use web_sys::Element;
use yew::prelude::*;

// Note: This name conflicts with yew_router::switch::Switch.
// Not sure what to think about that.
pub struct Switch {
    node_ref: NodeRef,
    link: ComponentLink<Self>,
    inner: Option<MDCSwitch>,
    props: Props,
    state: bool,
}

#[derive(Properties, Clone, PartialEq)]
pub struct Props {
    #[prop_or_default]
    pub id: String,
    #[prop_or_default]
    pub state: bool,
    #[prop_or_default]
    pub label_text: String,
    #[prop_or_else(Callback::noop)]
    pub onchange: Callback<bool>,
}

pub enum Msg {
    StateChanged,
}

impl Component for Switch {
    type Message = Msg;
    type Properties = Props;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self {
            node_ref: NodeRef::default(),
            link,
            inner: None,
            state: props.state,
            props,
        }
    }

    fn rendered(&mut self, first_render: bool) {
        if first_render {
            if let Some(inner) = self.inner.take() {
                inner.destroy();
            }
            self.inner = self.node_ref.cast::<Element>().map(MDCSwitch::new);
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
            Msg::StateChanged => {
                self.state = !self.state;
                self.props.onchange.emit(self.state);
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
            <div id=&self.props.id class=classes ref=self.node_ref.clone()>
                <div class="mdc-switch__track"></div>
                <div class="mdc-switch__thumb-underlay">
                    <div class="mdc-switch__thumb">
                        <input type="checkbox"
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
                <label>{ &self.props.label_text }</label>
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
