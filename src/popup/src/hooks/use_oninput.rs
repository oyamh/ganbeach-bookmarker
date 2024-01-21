use gloo_events::EventListener;
use web_sys::HtmlInputElement;
use yew::prelude::*;

#[hook]
pub fn use_oninput<T, D>(input_ref: &NodeRef, callback: T, dependents: D)
where
    T: Fn(&Event) + 'static,
    D: PartialEq + 'static,
{
    let input_ref = input_ref.clone();
    use_effect_with_deps(
        move |_| {
            // let input = input_ref.cast::<HtmlInputElement>();
            let listener = if let Some(input) = input_ref.cast::<HtmlInputElement>() {
                let listener = EventListener::new(&input, "input", move |e| {
                    callback(e);
                });
                Some(listener)
            } else {
                None
            };
            || drop(listener)
        },
        dependents,
    );
}
