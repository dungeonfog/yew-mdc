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

#[cfg(feature = "drawer")]
#[wasm_bindgen(module = "@material/drawer/index")]
extern "C" {
    #[wasm_bindgen(extends = MDCComponent)]
    pub type MDCDrawer;

    #[wasm_bindgen(constructor)]
    pub fn new(surface: Element) -> MDCDrawer;

    #[wasm_bindgen(method, getter)]
    pub fn open(this: &MDCDrawer) -> bool;
    #[wasm_bindgen(method, setter)]
    pub fn set_open(this: &MDCDrawer, open: bool);
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

    #[wasm_bindgen(method)]
    pub fn focus(this: &MDCTextField);
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
    pub fn set_timeout_ms(this: &MDCSnackbar, timeout_ms: i16);

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

#[cfg(feature = "switch")]
#[wasm_bindgen(module = "@material/switch/index")]
extern "C" {
    #[wasm_bindgen(extends = MDCComponent)]
    pub type MDCSwitch;

    #[wasm_bindgen(constructor)]
    pub fn new(surface: Element) -> MDCSwitch;

    #[wasm_bindgen(method, getter, js_name = checked)]
    pub fn checked(this: &MDCSwitch) -> bool;
    #[wasm_bindgen(method, setter, js_name = checked)]
    pub fn set_checked(this: &MDCSwitch, checked: bool);

    #[wasm_bindgen(method, getter, js_name = disabled)]
    pub fn disabled(this: &MDCSwitch) -> bool;
    #[wasm_bindgen(method, setter, js_name = disabled)]
    pub fn set_disabled(this: &MDCSwitch, disabled: bool);
}

#[cfg(feature = "list")]
#[wasm_bindgen(module = "@material/list")]
extern "C" {
    #[wasm_bindgen(extends = MDCComponent)]
    pub type MDCList;

    #[wasm_bindgen(constructor)]
    pub fn new(surface: Element) -> MDCList;

    #[wasm_bindgen(method, setter = vertical, structural)]
    pub fn set_vertical(this: &MDCList, flag: bool);

    #[wasm_bindgen(method, getter = listElements, structural)]
    pub fn list_elements(this: &MDCList) -> js_sys::Array;

    #[wasm_bindgen(method, setter = wrapFocus, structural)]
    pub fn set_wrap_focus(this: &MDCList, flag: bool);

    #[wasm_bindgen(method, setter = singleSelection, structural)]
    pub fn set_single_selection(this: &MDCList, flag: bool);

    #[wasm_bindgen(method, getter = selectedIndex, structural)]
    pub fn selected_index(this: &MDCList) -> i32;

    #[wasm_bindgen(method, setter = selectedIndex, structural)]
    pub fn set_selected_index(this: &MDCList, value: i32);
}

#[cfg(feature = "slider")]
#[wasm_bindgen(module = "@material/slider")]
extern "C" {
    #[wasm_bindgen(extends = MDCComponent)]
    pub type MDCSlider;

    #[wasm_bindgen(constructor)]
    pub fn new(surface: Element) -> MDCSlider;

    #[wasm_bindgen(method, setter = value, structural)]
    pub fn set_value(this: &MDCSlider, value: f64);

    #[wasm_bindgen(method, getter, structural)]
    pub fn value(this: &MDCSlider) -> f64;

    #[wasm_bindgen(method, setter = min, structural)]
    pub fn set_min(this: &MDCSlider, value: f64);

    #[wasm_bindgen(method, getter, structural)]
    pub fn min(this: &MDCSlider) -> f64;

    #[wasm_bindgen(method, setter = max, structural)]
    pub fn set_max(this: &MDCSlider, value: f64);

    #[wasm_bindgen(method, getter, structural)]
    pub fn max(this: &MDCSlider) -> f64;

    #[wasm_bindgen(method, setter = step, structural)]
    pub fn set_step(this: &MDCSlider, value: f64);

    #[wasm_bindgen(method, getter, structural)]
    pub fn step(this: &MDCSlider) -> f64;

    #[wasm_bindgen(method, structural)]
    pub fn layout(this: &MDCSlider);

    #[wasm_bindgen(method, structural, js_name = stepUp)]
    pub fn step_up(this: &MDCSlider, amount: f64);

    #[wasm_bindgen(method, structural, js_name = stepDown)]
    pub fn step_down(this: &MDCSlider, amount: f64);
}

#[cfg(feature = "data-table")]
#[wasm_bindgen(module = "@material/data-table")]
extern "C" {
    #[wasm_bindgen(extends = MDCComponent)]
    pub type MDCDataTable;

    #[wasm_bindgen(constructor)]
    pub fn new(surface: Element) -> MDCDataTable;

    #[wasm_bindgen(method, structural)]
    pub fn layout(this: &MDCDataTable);

    #[wasm_bindgen(method, structural, js_name = getRows)]
    pub fn get_rows(this: &MDCDataTable) -> js_sys::Array;

    #[wasm_bindgen(method, structural, js_name = getSelectedRowIds)]
    pub fn get_selected_row_ids(this: &MDCDataTable) -> js_sys::Array;

    #[wasm_bindgen(method, structural, js_name = setSelectedRowIds)]
    pub fn set_selected_row_ids(this: &MDCDataTable, row_ids: js_sys::Array);
}

#[cfg(feature = "tabs")]
#[wasm_bindgen(module = "@material/tab")]
extern "C" {
    #[wasm_bindgen(extends = MDCComponent)]
    pub type MDCTab;

    #[wasm_bindgen(constructor)]
    pub fn new(surface: Element) -> MDCTab;

    #[wasm_bindgen(method, getter)]
    pub fn id(this: &MDCTab) -> String;
    #[wasm_bindgen(method, setter = id)]
    pub fn set_id(this: &MDCTab, id: &str);

    #[wasm_bindgen(method, getter)]
    pub fn active(this: &MDCTab) -> bool;

    #[wasm_bindgen(method, getter = focusOnActivate)]
    pub fn focus_on_activate(this: &MDCTab) -> bool;

    #[wasm_bindgen(method)]
    pub fn activate(this: &MDCTab);
    // Do we even need this variant?
    #[wasm_bindgen(method, js_name = activate)]
    pub fn activate_with_previous_indicator_client_rect(
        this: &MDCTab,
        previous_indicator_client_rect: JsValue,
    );

    #[wasm_bindgen(method)]
    pub fn deactivate(this: &MDCTab);

    #[wasm_bindgen(method)]
    pub fn focus(this: &MDCTab);

    #[wasm_bindgen(method, js_name = computeIndicatorClientRect)]
    pub fn compute_indicator_client_rect(this: &MDCTab) -> JsValue;

    #[wasm_bindgen(method, js_name = computeDimensions)]
    pub fn compute_dimensions(this: &MDCTab) -> JsValue;
}

#[cfg(feature = "tabs")]
#[wasm_bindgen(module = "@material/tab-bar")]
extern "C" {
    #[wasm_bindgen(extends = MDCComponent)]
    pub type MDCTabBar;

    #[wasm_bindgen(constructor)]
    pub fn new(surface: Element) -> MDCTabBar;

    #[wasm_bindgen(method, setter = focusOnActivate)]
    pub fn focus_on_activate(this: &MDCTabBar, focus_on_active: bool);

    #[wasm_bindgen(method, setter = useAutomaticActivation)]
    pub fn use_automatic_activation(this: &MDCTabBar, use_automatic_activation: bool);

    #[wasm_bindgen(method, js_name = activateTab)]
    pub fn activate_tab(this: &MDCTabBar, index: u32);

    #[wasm_bindgen(method, js_name = scrollIntoView)]
    pub fn scroll_into_view(this: &MDCTabBar, tab_index: u32);
}

#[cfg(feature = "tabs")]
#[wasm_bindgen(module = "@material/tab-indicator")]
extern "C" {
    #[wasm_bindgen(extends = MDCComponent)]
    pub type MDCTabIndicator;

    #[wasm_bindgen(constructor)]
    pub fn new(surface: Element) -> MDCTabIndicator;

    #[wasm_bindgen(method)]
    pub fn activate(this: &MDCTabIndicator);
    // Do we even need this variant?
    #[wasm_bindgen(method, js_name = activate)]
    pub fn activate_with_previous_indicator_client_rect(
        this: &MDCTabIndicator,
        previous_indicator_client_rect: JsValue,
    );

    #[wasm_bindgen(method)]
    pub fn deactivate(this: &MDCTabIndicator);

    #[wasm_bindgen(method, js_name = computeContentClientRect)]
    pub fn compute_content_client_rect(this: &MDCTabIndicator) -> JsValue;
}

#[cfg(feature = "tabs")]
#[wasm_bindgen(module = "@material/tab-scroller")]
extern "C" {
    #[wasm_bindgen(extends = MDCComponent)]
    pub type MDCTabScroller;

    #[wasm_bindgen(constructor)]
    pub fn new(surface: Element) -> MDCTabScroller;

    #[wasm_bindgen(method, js_name = scrollTo)]
    pub fn scroll_to(this: &MDCTabScroller, scroll_x: i32);

    #[wasm_bindgen(method, js_name = incrementScroll)]
    pub fn increment_scroll(this: &MDCTabScroller, scroll_x: i32);

    #[wasm_bindgen(method, js_name = incrementScrollImmediate)]
    pub fn increment_scroll_immediate(this: &MDCTabScroller, scroll_x: i32);

    #[wasm_bindgen(method, js_name = getScrollPosition)]
    pub fn get_scroll_position(this: &MDCTabScroller) -> i32;

    #[wasm_bindgen(method, js_name = getScrollContentWidth)]
    pub fn get_scroll_content_width(this: &MDCTabScroller) -> i32;
}
