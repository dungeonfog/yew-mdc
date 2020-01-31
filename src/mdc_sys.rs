use wasm_bindgen::prelude::*;
use web_sys::Element;

#[wasm_bindgen(module = "@material/base/component/index")]
extern "C" {
    pub type MDCComponent;

    #[wasm_bindgen(method)]
    pub fn listen(this: &MDCComponent, type_: &str, handler: &Closure<dyn FnMut(web_sys::Event)>);

    #[wasm_bindgen(method)]
    pub fn unlisten(this: &MDCComponent, type_: &str, handler: &Closure<dyn FnMut(web_sys::Event)>);

    #[wasm_bindgen(method)]
    pub fn destroy(this: &MDCComponent);
}

#[cfg(any(feature = "button", feature = "fab"))]
#[wasm_bindgen(module = "@material/ripple/index")]
extern "C" {
    /// MDC Ripple provides the JavaScript and CSS required to provide components
    /// (or any element at all) with a material “ink ripple” interaction effect.
    /// It is designed to be efficient, uninvasive, and usable without adding
    /// any extra DOM to your elements.
    ///
    /// MDC Ripple also works without JavaScript, where it gracefully degrades
    /// to a simpler CSS-Only implementation.
    #[wasm_bindgen(extends = MDCComponent)]
    pub type MDCRipple;

    #[wasm_bindgen(constructor)]
    pub fn new(surface: Element) -> MDCRipple;

    #[wasm_bindgen(method, getter)]
    pub fn unbounded(this: &MDCRipple) -> bool;
    #[wasm_bindgen(method, setter)]
    pub fn set_unbounded(this: &MDCRipple, unbounded: bool);

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
}

#[cfg(feature = "text-field")]
#[wasm_bindgen(module = "@material/textfield/index")]
extern "C" {
    /// Text fields allow users to input, edit, and select text.
    #[wasm_bindgen(extends = MDCComponent)]
    pub type MDCTextField;

    #[wasm_bindgen(constructor)]
    pub fn new(surface: Element) -> MDCTextField;

    #[wasm_bindgen(method, getter)]
    pub fn value(this: &MDCTextField) -> String;
    #[wasm_bindgen(method, setter)]
    pub fn set_value(this: &MDCTextField, value: &str);

    #[wasm_bindgen(method, getter)]
    pub fn disabled(this: &MDCTextField) -> bool;
    #[wasm_bindgen(method, setter)]
    pub fn set_disabled(this: &MDCTextField, disabled: bool);
}

#[cfg(feature = "menu")]
#[wasm_bindgen(module = "@material/menu/index")]
extern "C" {
    /// A menu displays a list of choices on a temporary surface.
    /// They appear when users interact with a button, action, or other control.
    #[wasm_bindgen(extends = MDCComponent)]
    pub type MDCMenu;

    #[wasm_bindgen(constructor)]
    pub fn new(surface: Element) -> MDCMenu;

    #[wasm_bindgen(method, getter)]
    pub fn open(this: &MDCMenu) -> bool;
    #[wasm_bindgen(method, setter)]
    pub fn set_open(this: &MDCMenu, open: bool);

    #[wasm_bindgen(method, getter)]
    pub fn items(this: &MDCMenu) -> js_sys::Array;

    #[wasm_bindgen(method, getter, js_name = quickOpen)]
    pub fn quick_open(this: &MDCMenu) -> bool;
    #[wasm_bindgen(method, setter, js_name = quickOpen)]
    pub fn set_quick_open(this: &MDCMenu, quick_open: bool);

    #[wasm_bindgen(method, getter, js_name = wrapFocus)]
    pub fn wrap_focus(this: &MDCMenu) -> bool;
    #[wasm_bindgen(method, setter, js_name = wrapFocus)]
    pub fn set_wrap_focus(this: &MDCMenu, wrap_focus: bool);

    #[wasm_bindgen(method, js_name = setAnchorCorner)]
    pub fn set_anchor_corner(this: &MDCMenu, corner: js_sys::Object);
    #[wasm_bindgen(method, js_name = setAnchorMargin)]
    pub fn set_anchor_margin(this: &MDCMenu, distance: js_sys::Object);

    #[wasm_bindgen(method, js_name = setAbsolutePosition)]
    pub fn set_absolute_position(this: &MDCMenu, x: i32, y: i32);

    #[wasm_bindgen(method, js_name = setFixedPosition)]
    pub fn set_fixed_position(this: &MDCMenu, is_fixed: bool);

    #[wasm_bindgen(method, js_name = setSelectedIndex)]
    pub fn set_selected_index(this: &MDCMenu, index: u32);

    #[wasm_bindgen(method, js_name = setIsHoisted)]
    pub fn set_is_hoisted(this: &MDCMenu, is_hoisted: bool);

    #[wasm_bindgen(method, js_name = setAnchorElement)]
    pub fn set_anchor_element(this: &MDCMenu, element: Element);

    #[wasm_bindgen(method, js_name = getOptionByIndex)]
    pub fn get_option_by_index(this: &MDCMenu, index: u32) -> Option<Element>;

    #[wasm_bindgen(method, js_name = setEnabled)]
    pub fn set_enabled(this: &MDCMenu, index: u32, is_enabled: bool);
}

#[cfg(feature = "menu")]
#[wasm_bindgen(module = "@material/menu-surface/index")]
extern "C" {
    /// The MDC Menu Surface component is a reusable surface that appears above
    /// the content of the page and can be positioned adjacent to an element.
    /// Menu Surfaces require JavaScript to properly position themselves when opening.
    #[wasm_bindgen(extends = MDCComponent)]
    pub type MDCMenuSurface;

    #[wasm_bindgen(constructor)]
    pub fn new(surface: Element) -> MDCMenuSurface;
}

#[cfg(feature = "dialog")]
#[wasm_bindgen(module = "@material/dialog/index")]
extern "C" {
    #[wasm_bindgen(extends = MDCComponent)]
    pub type MDCDialog;

    #[wasm_bindgen(constructor)]
    pub fn new(surface: Element) -> MDCDialog;

    #[wasm_bindgen(method, getter, js_name = isOpen)]
    pub fn is_open(this: &MDCDialog) -> bool;

    #[wasm_bindgen(method, getter, js_name = escapeKeyAction)]
    pub fn escape_key_action(this: &MDCDialog) -> String;
    #[wasm_bindgen(method, setter, js_name = escapeKeyAction)]
    pub fn set_escape_key_action(this: &MDCDialog, action: &str);

    #[wasm_bindgen(method, getter, js_name = scrimClickAction)]
    pub fn scrim_click_action(this: &MDCDialog) -> String;
    #[wasm_bindgen(method, setter, js_name = scrimClickAction)]
    pub fn set_scrim_click_action(this: &MDCDialog, action: &str);

    #[wasm_bindgen(method, getter, js_name = autoStackButtons)]
    pub fn auto_stack_buttons(this: &MDCDialog) -> bool;
    #[wasm_bindgen(method, setter, js_name = autoStackButtons)]
    pub fn set_auto_stack_buttons(this: &MDCDialog, auto_stack_buttons: bool);

    #[wasm_bindgen(method)]
    pub fn layout(this: &MDCDialog);

    #[wasm_bindgen(method)]
    pub fn open(this: &MDCDialog);

    #[wasm_bindgen(method)]
    pub fn close(this: &MDCDialog, action: Option<&str>);
}

#[cfg(feature = "snackbar")]
#[wasm_bindgen(module = "@material/snackbar/index")]
extern "C" {
    #[wasm_bindgen(extends = MDCComponent)]
    pub type MDCSnackbar;

    #[wasm_bindgen(constructor)]
    pub fn new(surface: Element) -> MDCSnackbar;

    #[wasm_bindgen(method, getter, js_name = "isOpen")]
    pub fn is_open(this: &MDCSnackbar) -> bool;

    #[wasm_bindgen(method, getter, js_name = timeoutMs)]
    pub fn timeout_ms(this: &MDCSnackbar) -> u16;
    #[wasm_bindgen(method, setter, js_name = timeoutMs)]
    pub fn set_timeout_ms(this: &MDCSnackbar, timeout_ms: u16);

    #[wasm_bindgen(method, getter, js_name = closeOnEscape)]
    pub fn close_on_escape(this: &MDCSnackbar) -> bool;
    #[wasm_bindgen(method, setter, js_name = closeOnEscape)]
    pub fn set_close_on_escape(this: &MDCSnackbar, close_on_escape: bool);

    #[wasm_bindgen(method, getter, js_name = labelText)]
    pub fn label_text(this: &MDCSnackbar) -> String;
    #[wasm_bindgen(method, setter, js_name = labelText)]
    pub fn set_label_text(this: &MDCSnackbar, label_text: &str);

    #[wasm_bindgen(method, getter, js_name = actionButtonText)]
    pub fn action_button_text(this: &MDCSnackbar) -> String;
    #[wasm_bindgen(method, setter, js_name = actionButtonText)]
    pub fn set_action_button_text(this: &MDCSnackbar, action_button_text: &str);

    #[wasm_bindgen(method)]
    pub fn open(this: &MDCSnackbar);
    #[wasm_bindgen(method)]
    pub fn close(this: &MDCSnackbar, reason: Option<&str>);
}
