/*!
[Material Image List](https://material.io/develop/web/components/image-lists/)

This is a pure CSS component and does not need any JavaScript.

General usage:

```
# use yew::html;
use yew_mdc::components::image_list::{
    ImageList, Item, Image, Supporting,
};
html! {
    // The root ImageList compiles to an <ul> element...
    <ImageList>
        // The Items are <li> elements.
        <Item>
            // The Image component creates both an img tag and the
            // aspect container around it.
            // If you want a masonry image list, don't use Image;
            // use a normal img tag instead.
            <Image src="/img/some_image.jpg" />
        </Item>
        <Item>
            <Image src="/img/some_other_image.png" />
            // And the Supporting component is a pure wrapper.
            // (<div class="mdc-image-list__supporting">)
            <Supporting>
                <span class="mdc-image-list__label">{ "Some text here" }</span>
            </Supporting>
        </Item>
    </ImageList>
};
```

Use the list according to the MDC docs:

```scss
// In style.scss
@import "@material/image-list";
.my-image-list {
    @include image-list.standard-columns(5);
}
```

```
# use yew_mdc::components::ImageList;
# use yew::html;
// In my_component.rs
html! {
    <ImageList classes="my-image-list">
        // Various items...
        // Note: An empty ImageList will fail to compile;
        // it requires at least one item.
#        <div></div>
    </ImageList>
};
```
*/

use yew::prelude::*;

pub mod item;
pub use item::{Image, Item, Supporting};

pub struct ImageList;

#[derive(Properties, Clone, PartialEq)]
pub struct Props {
    pub children: Children,
    #[prop_or_default]
    pub id: String,
    #[prop_or_default]
    pub classes: String,
}

impl Component for ImageList {
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
        html! {
            <ul id={ctx.props().id.clone()} class={format!("mdc-image-list {}", ctx.props().classes)}>
                { ctx.props().children.clone() }
            </ul>
        }
    }
}
