use crate::mdc_sys::MDCSwitch;
use web_sys::Element;
use yew::prelude::*;

// Note: This name conflicts with yew_router::switch::Switch.
// Not sure what to think about that.
pub struct Switch {
    node_ref: NodeRef,
    inner: Option<MDCSwitch>,
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

    fn create(ctx: &Context<Self>) -> Self {
        Self {
            node_ref: NodeRef::default(),
            inner: None,
            state: ctx.props().state,
        }
    }

    fn rendered(&mut self, _ctx: &Context<Self>, first_render: bool) {
        if first_render {
            if let Some(inner) = self.inner.take() {
                inner.destroy();
            }
            self.inner = self.node_ref.cast::<Element>().map(MDCSwitch::new);
        }
    }

    fn changed(&mut self, _ctx: &Context<Self>) -> bool {
        true
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::StateChanged => {
                self.state = !self.state;
                if let Some(ref inner) = self.inner {
                    inner.set_checked(self.state);
                }
                ctx.props().onchange.emit(self.state);
                true
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let emit_change = ctx.link().callback(|_| Msg::StateChanged);
        let (on_class, checked) = if self.state {
            (" mdc-switch--checked", true)
        } else {
            ("", false)
        };
        let classes = format!("mdc-switch{}", on_class);
        let switch = html! {
            <div id={ctx.props().id.clone()} class={classes} ref={self.node_ref.clone()}>
                <div class="mdc-switch__track"></div>
                <div class="mdc-switch__thumb-underlay">
                    <div class="mdc-switch__thumb">
                        <input type="checkbox"
                               onchange={emit_change}
                               class="mdc-switch__native-control"
                               checked={checked}
                            />
                    </div>
                </div>
            </div>
        };
        if ctx.props().label_text.is_empty() {
            switch
        } else {
            html! {
                <>
                { switch }
                <label>{ &ctx.props().label_text }</label>
                </>
            }
        }
    }

    fn destroy(&mut self, _ctx: &Context<Self>) {
        if let Some(ref inner) = self.inner {
            inner.destroy();
        }
    }
}
