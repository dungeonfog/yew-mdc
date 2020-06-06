/*!
Icons buttons, as defined [here](https://material.io/develop/web/components/buttons/icon-buttons/).

For normal buttons, there's nothing to take note about. They work similar to
regular buttons with icons, except that you can also pass them a class and a
simple string as the content, to make this material icons example possible:

```xml
<IconButton classes="material-icons">{ "done" }</IconButton>
```

If you omit the `classes` property, you just get a `<button class="mdc-icon-button">`
and can put whatever you want inside.

For **toggle buttons**, the situation is a bit more complicated: Currently,
you have to make sure you put exactly the right contents inside (as defined
[here](https://material.io/develop/web/components/buttons/icon-buttons/#icon-button-toggle)).

This could, for example, look like this:

```xml
<IconButton togglable=true toggle_on=true>
    <i class="material-icons mdc-icon-button__icon mdc-icon-button__icon--on">{ "favorite" }</i>
    <i class="material-icons mdc-icon-button__icon">{ "favorite_border" }</i>
</IconButton>
```

Setting `toggle_on` to `true` will then show the icon which has the `mdc-icon-button__icon--on`
class, while setting it to `false` will show the other icon.

The plus side of this is that it allows full customizability, as long as you
stay in the realm of MDC; the downside is that it's also on you to get it right.
*/

use crate::mdc_sys::MDCRipple;
use web_sys::Element;
use yew::prelude::*;

pub struct IconButton {
    node_ref: NodeRef,
    ripple: Option<MDCRipple>,
    props: Props,
    link: ComponentLink<Self>,
}

#[derive(Properties, Clone, PartialEq)]
pub struct Props {
    pub children: Children,
    #[prop_or_default]
    pub id: String,
    #[prop_or_default]
    pub classes: String,
    #[prop_or_default]
    pub togglable: bool,
    #[prop_or_default]
    pub toggle_on: bool,
    #[prop_or_default]
    pub disabled: bool,
    #[prop_or_else(Callback::noop)]
    pub onclick: Callback<MouseEvent>,
}

pub enum Msg {
    Clicked(MouseEvent),
}

impl Component for IconButton {
    type Message = Msg;
    type Properties = Props;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self {
            node_ref: NodeRef::default(),
            ripple: None,
            props,
            link,
        }
    }

    fn rendered(&mut self, first_render: bool) {
        if first_render {
            if let Some(ripple) = self.ripple.take() {
                ripple.destroy();
            }
            self.ripple = self.node_ref.cast::<Element>().map(|elem| {
                let ripple = MDCRipple::new(elem);
                ripple.set_unbounded(true);
                ripple
            });
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
            Msg::Clicked(event) => {
                self.props.onclick.emit(event);
            }
        }
        false
    }

    fn view(&self) -> Html {
        let onclick = self.link.callback(Msg::Clicked);
        if self.props.togglable {
            let on = if self.props.toggle_on {
                " mdc-icon-button--on"
            } else {
                ""
            };
            let classes = format!("mdc-icon-button{}", on);
            html! {
                <button id=&self.props.id class=classes
                        ref=self.node_ref.clone()
                        onclick=onclick
                        disabled=self.props.disabled>
                    { self.props.children.render() }
                </button>
            }
        } else {
            let classes = format!("mdc-icon-button {}", self.props.classes);
            html! {
                <button class=classes id=&self.props.id
                        ref=self.node_ref.clone()
                        onclick=onclick
                        disabled=self.props.disabled>
                    { self.props.children.render() }
                </button>
            }
        }
    }

    fn destroy(&mut self) {
        if let Some(ripple) = &self.ripple {
            ripple.destroy();
        }
    }
}
