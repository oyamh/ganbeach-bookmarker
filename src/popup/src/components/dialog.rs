use web_sys::HtmlDialogElement;
use yew::prelude::*;

use crate::hooks::use_dialog_ref::DialogValue;

const CSS_DIALOG: &'static str = include_str!("../css/dialog.css");

#[derive(Properties, PartialEq)]
pub struct Props {
    pub dialog_ref: NodeRef,
}

#[function_component(Dialog)]
pub fn dialog(props: &Props) -> Html {
    let Props { dialog_ref } = props;

    let ok = {
        let dialog_ref = dialog_ref.clone();
        move |e: MouseEvent| {
            e.prevent_default();
            if let Some(dialog) = dialog_ref.cast::<HtmlDialogElement>() {
                dialog.close_with_return_value(DialogValue::OK.into())
            }
        }
    };

    let cancel = {
        let dialog_ref = dialog_ref.clone();
        move |e: MouseEvent| {
            e.prevent_default();
            if let Some(dialog) = dialog_ref.cast::<HtmlDialogElement>() {
                dialog.close_with_return_value(DialogValue::Cancel.into())
            }
        }
    };

    html! {
        <>
        <style>
            {CSS_DIALOG}
        </style>
        <dialog id="dialog-container" ref={dialog_ref.clone()}>
            <div id="dialog-wrapper">
                <form>
                    <div id="dialog-label">
                        {"Create new folder"}
                    </div>

                    <div class={classes!("buttons-container")}>
                        <div class={classes!("buttons-wrapper")}>

                            <div class={classes!("button-container")} title={"OK"}>
                                <div class={classes!("button-wrapper", "ok-button")}>
                                    <button class={classes!("button")} value="ok" onclick={ok} type="button">{"OK"}</button>
                                </div>
                            </div>

                            <div class={classes!("button-container")} title={"Cancel"}>
                                <div class={classes!("button-wrapper", "cancel-button")}>
                                    <button class={classes!("button")} value="cancel" onclick={cancel} type="button">{"Cancel"}</button>
                                </div>
                            </div>

                        </div>
                    </div>

                </form>
            </div>
        </dialog>
        </>
    }
}
