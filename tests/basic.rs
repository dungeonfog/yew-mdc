use futures::prelude::*;
use wasm_bindgen::JsValue;
use wasm_bindgen_futures::JsFuture;
use wasm_bindgen_test::{wasm_bindgen_test, wasm_bindgen_test_configure};

wasm_bindgen_test_configure!(run_in_browser);

// This test currently doesn't run; I need to figure out how to include JS components.
//#[wasm_bindgen_test]
//fn web_test() {
//    use yew::{html, virtual_dom::VNode};
//    use yew_mdc::components::*;
//    let button = html! {
//        <Button text="Button" />
//    };
//    assert!(match button {
//        VNode::VComp(_) => true,
//        _ => false,
//    })
//}

#[wasm_bindgen_test]
async fn async_test() {
    let promise = js_sys::Promise::resolve(&JsValue::from(42));

    JsFuture::from(promise)
        .map(|x| assert_eq!(x.unwrap(), 42))
        .await
}
