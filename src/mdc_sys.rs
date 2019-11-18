use wasm_bindgen::prelude::*;
use web_sys::Element;

#[cfg(any(feature = "mdc-button", feature = "mdc-fab"))]
#[wasm_bindgen(module = "@material/ripple/index")]
extern "C" {
    pub type MDCRipple;

    #[wasm_bindgen(constructor)]
    pub fn new(surface: Element) -> MDCRipple;

    #[wasm_bindgen(method)]
    pub fn activate(this: &MDCRipple);

    #[wasm_bindgen(method)]
    pub fn deactivate(this: &MDCRipple);

    #[wasm_bindgen(method)]
    pub fn layout(this: &MDCRipple);

    #[wasm_bindgen(method)]
    pub fn handle_focus(this: &MDCRipple);

    #[wasm_bindgen(method)]
    pub fn handle_blur(this: &MDCRipple);

    #[wasm_bindgen(method)]
    pub fn destroy(this: &MDCRipple);
}

#[cfg(feature = "mdc-text-field")]
#[wasm_bindgen(module = "@material/textfield/index")]
extern "C" {
    /// Text fields allow users to input, edit, and select text.
    pub type MDCTextField;

    #[wasm_bindgen(constructor)]
    pub fn new(surface: Element) -> MDCTextField;

    #[wasm_bindgen(method, getter)]
    pub fn value(this: &MDCTextField) -> String;

    #[wasm_bindgen(method, setter)]
    pub fn set_value(this: &MDCTextField, value: String);

    #[wasm_bindgen(method, getter)]
    pub fn disabled(this: &MDCTextField) -> bool;

    #[wasm_bindgen(method, setter)]
    pub fn set_disabled(this: &MDCTextField, disabled: bool);

    #[wasm_bindgen(method)]
    pub fn destroy(this: &MDCTextField);
}

// Commented out for now - might want to revisit later in case we need more
// sophisticated text fields.
// #[cfg(feature = "mdc-text-field")]
// #[wasm_bindgen(module = "@material/line-ripple/index")]
// extern "C" {
//     /// The line ripple is used to highlight user-specified input above it.
//     /// When a line ripple is active, the line’s color and thickness changes.
//     pub type MDCLineRipple;

//     #[wasm_bindgen(constructor)]
//     pub fn new(surface: Element) -> MDCLineRipple;

//     #[wasm_bindgen(method)]
//     pub fn activate(this: &MDCLineRipple);

//     #[wasm_bindgen(method)]
//     pub fn deactivate(this: &MDCLineRipple);

//     #[wasm_bindgen(method)]
//     pub fn set_ripple_center(this: &MDCLineRipple, xCoordinate: js_sys::Number);

//     #[wasm_bindgen(method)]
//     pub fn destroy(this: &MDCLineRipple);
// }

// #[cfg(feature = "mdc-text-field")]
// #[wasm_bindgen(module = "@material/floating-label/index")]
// extern "C" {
//     /// Floating labels display the type of input a field requires.
//     /// Every Text Field and Select should have a label, except for full-width
//     /// text fields, which use the input’s `placeholder` attribute instead.
//     /// Labels are aligned with the input line and always visible.
//     /// They can be resting (when a field is inactive and empty) or floating.
//     /// The label is a text caption or description for the Text Field.
//     pub type MDCFloatingLabel;

//     #[wasm_bindgen(constructor)]
//     pub fn new(surface: Element) -> MDCFloatingLabel;

//     #[wasm_bindgen(method)]
//     pub fn shake(this: &MDCFloatingLabel, should_shake: bool);

//     #[wasm_bindgen(method)]
//     pub fn float(this: &MDCFloatingLabel, should_float: bool);

//     #[wasm_bindgen(method)]
//     pub fn get_width(this: &MDCFloatingLabel) -> js_sys::Number;

//     #[wasm_bindgen(method)]
//     pub fn destroy(this: &MDCFloatingLabel);
// }
