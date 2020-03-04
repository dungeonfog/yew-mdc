/*!
Yew components wrapping around Google's Material Design Components (MDC).

This is a work in progress at the moment - no stability guarantees of any kind
until I have decided on both the public API and the features I want to expose.

What you can expect right now:
- You can find the implemented components at `yew_mdc::components`.
- They are named after their MDC *CSS classes* in PascalCase; for example,
  floating action buttons (`mdc-fab`) will be called `FAB`, while text fields
  (`mdc-text-field`) will be called `TextField`. Their respective *modules*
  have the same names in snake_case.

And in the future:
- Each component struct is re-exported from the top components module; some
  will have additional data structures associated with them, which should be
  documented at their module-level at some point.
- Each module-level and struct component documentations should also point out
  the CSS classes and JavaScript dependencies you need to add to your project.

Note that this project currently assumes babel is used (it tries to import
`@material/$component/index`).

Also note that the JS bindings here are 100% `wasm-bindgen` - this means that
we can't use some parts of Yew and have to rely on element ids to find the
right element to attach an MDC component to.

You can find the raw MDC bindings at `yew_mdc::mdc_sys`. These might be
published as a separate crate at some point.

Pretty much all of the components are crate features, so you can only pull in
what you actually need; currently, all features are enabled by default.
*/
#![recursion_limit = "256"]

pub mod components;
pub mod mdc_sys;

// #[cfg(test)]
// pub mod tests {
//     use wasm_bindgen_test::{wasm_bindgen_test, wasm_bindgen_test_configure};
//     wasm_bindgen_test_configure!(run_in_browser);

//     // #[wasm_bindgen_test]
//     // fn id_generation() {
//     //     assert_eq!(super::next_id(), 0);
//     //     assert_eq!(super::next_id(), 1);
//     //     assert_eq!(super::next_id(), 2);
//     // }
// }
