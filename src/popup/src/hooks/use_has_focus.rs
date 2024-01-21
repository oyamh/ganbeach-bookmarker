use gloo_events::EventListener;
use web_sys::HtmlInputElement;
use yew::prelude::*;

#[hook]
pub fn use_has_focus<D>(input_ref: &NodeRef, dependents: D) -> UseStateHandle<bool>
where
    D: PartialEq + 'static,
{
    let has_focus = use_state(|| false);
    {
        let has_focus = has_focus.clone();
        let input_ref = input_ref.clone();
        use_effect_with_deps(
            move |_| {
                let listeners = if let Some(input) = input_ref.cast::<HtmlInputElement>() {
                    let on_focus = {
                        let has_focus = has_focus.clone();
                        EventListener::new(&input, "focus", move |_e| {
                            has_focus.set(true);
                        })
                    };
                    let on_blur = {
                        let has_focus = has_focus.clone();
                        EventListener::new(&input, "blur", move |_e| {
                            has_focus.set(false);
                        })
                    };
                    Some((on_focus, on_blur))
                } else {
                    None
                };
                || drop(listeners)
            },
            dependents,
        );
    }
    has_focus
}
