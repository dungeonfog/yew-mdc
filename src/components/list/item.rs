use std::fmt::Display;

use web_sys::Element;
use yew::prelude::*;

use crate::mdc_sys::MDCRipple;

#[derive(Clone, PartialEq)]
pub enum LineType {
    One,
    Two,
    Three,
}

impl Display for LineType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            LineType::One => f.write_str("mdc-list-item--with-one-line"),
            LineType::Two => f.write_str("mdc-list-item--with-two-lines"),
            LineType::Three => f.write_str("mdc-list-item--with-three-lines"),
        }
    }
}

impl Default for LineType {
    fn default() -> Self {
        LineType::One
    }
}

#[derive(Clone, PartialEq)]
pub enum LeadingType {
    Checkbox,
    Radio,
    Switch,
    Icon(String),
    Image,
    Thumbnail,
    Video,
    Avatar,
}

impl Into<Classes> for LeadingType {
    fn into(self) -> Classes {
        classes!(match self {
            LeadingType::Icon(_) => "mdc-list-item--with-leading-icon",
            LeadingType::Checkbox => "mdc-list-item--with-leading-checkbox",
            LeadingType::Radio => "mdc-list-item--with-leading-radio",
            LeadingType::Switch => "mdc-list-item--with-leading-switch",
            LeadingType::Image => "mdc-list-item--with-leading-image",
            LeadingType::Thumbnail => "mdc-list-item--with-leading-thumbnail",
            LeadingType::Video => "mdc-list-item--with-leading-video",
            LeadingType::Avatar => "mdc-list-item--with-leading-avatar",
        })
    }
}

pub struct Item {
    node_ref: NodeRef,
    ripple: Option<MDCRipple>,
}

pub enum Msg {
    Clicked(MouseEvent),
}

#[derive(Properties, Clone, PartialEq)]
pub struct Props {
    #[prop_or_default]
    pub id: String,
    #[prop_or_default]
    pub text: Vec<String>,
    #[prop_or_default]
    pub lines: LineType,
    #[prop_or_default]
    pub leading_item: Option<LeadingType>,
    #[prop_or_default]
    pub children: Children,
    #[prop_or_else(Callback::noop)]
    pub onclick: Callback<MouseEvent>,
}

impl Component for Item {
    type Message = Msg;
    type Properties = Props;

    fn create(_ctx: &Context<Self>) -> Self {
        Self {
            node_ref: NodeRef::default(),
            ripple: None,
        }
    }

    fn rendered(&mut self, _ctx: &Context<Self>, first_render: bool) {
        if first_render {
            if let Some(ripple) = self.ripple.take() {
                ripple.destroy();
            }
            self.ripple = self.node_ref.cast::<Element>().map(MDCRipple::new);
        }
    }

    fn changed(&mut self, _ctx: &Context<Self>) -> bool {
        true
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::Clicked(ev) => ctx.props().onclick.emit(ev),
        }
        false
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let Props {
            id,
            text,
            lines,
            leading_item,
            children,
            onclick,
        }: &Props = ctx.props();

        let classes = classes!["mdc-list-item", lines.to_string(), leading_item];
        let text = if text.is_empty() {
            html! {}
        } else {
            match lines {
                LineType::One => html! {
                    <span class="mdc-list-item__primary-text">{ &text[0] }</span>
                },
                LineType::Two => html! {
                    <>
                        <span class="mdc-list-item__primary-text">{ &text[0] }</span>
                        <span class="mdc-list-item__secondary-text">{ &text[1] }</span>
                    </>
                },
                LineType::Three => html! {
                    <>
                        <span class="mdc-list-item__primary-text">{ &text[0] }</span>
                        <span class="mdc-list-item__secondary-text">{ &text[1] }</span>
                        <span class="mdc-list-item__secondary-text">{ &text[2] }</span>
                    </>
                },
            }
        };
        let leading_item = if let Some(leading) = leading_item {
            html! {
                <span class="mdc-list-item__start">
                {
                    match leading {
                        LeadingType::Icon(icon) => html! {
                            <i class="material-icons">{ icon }</i>
                        },
                        _ => todo!()
                    }
                }
                </span>
            }
        } else {
            html! {}
        };
        html! {
            <li class={classes}
                onclick={onclick}
                ref={self.node_ref.clone()}>

                { children.clone() }
                <span class="mdc-list-item__ripple"></span>
                { leading_item }
                <span id={id.clone()} class="mdc-list-item__content">
                    { text }
                </span>
            </li>
        }
    }

    fn destroy(&mut self, _ctx: &Context<Self>) {
        if let Some(ripple) = &self.ripple {
            ripple.destroy();
        }
    }
}
