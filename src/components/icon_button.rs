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
use yew::prelude::*;

pub struct IconButton {
    id: String,
    ripple: Option<MDCRipple>,
    props: Props,
    link: ComponentLink<Self>,
}

#[derive(Properties, Clone)]
pub struct Props {
    pub id: Option<String>,
    pub classes: String,
    pub children: Children,
    pub togglable: bool,
    pub toggle_on: bool,
    pub onclick: Option<Callback<ClickEvent>>,
}

pub enum Msg {
    Clicked(ClickEvent),
}

impl Component for IconButton {
    type Message = Msg;
    type Properties = Props;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        let id = props
            .id
            .as_ref()
            .map(|id| id.to_owned())
            .unwrap_or_else(|| format!("icon-button-{}", crate::next_id()));
        Self {
            id,
            ripple: None,
            props,
            link,
        }
    }

    fn mounted(&mut self) -> ShouldRender {
        self.ripple = crate::get_element_by_id(&self.id).map(|elem| {
            let ripple = MDCRipple::new(elem);
            ripple.set_unbounded(true);
            ripple
        });
        false
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::Clicked(event) => {
                if let Some(callback) = &self.props.onclick {
                    callback.emit(event);
                }
            }
        }
        false
    }

    fn change(&mut self, props: Props) -> ShouldRender {
        if props.toggle_on != self.props.toggle_on {
            self.props = props;
            true
        } else {
            false
        }
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
                <button id=self.id class=classes onclick=onclick>
                    { self.props.children.render() }
                </button>
            }
        } else {
            let classes = format!("mdc-icon-button {}", self.props.classes);
            html! {
                <button class=classes id=self.id
                        onclick=onclick
                    >{ self.props.children.render() }</button>
            }
        }
    }

    fn destroy(&mut self) {
        if let Some(ripple) = &self.ripple {
            ripple.destroy();
        }
    }
}
