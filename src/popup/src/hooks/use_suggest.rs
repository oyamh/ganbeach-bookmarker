use crate::hooks::use_lists_context::use_lists_context;
use crate::hooks::use_oninput::use_oninput;
use domain::{FuzzyMatchCommand, SuggestCommand, SuggestResults, TypeCode};
use wasm_bindgen::JsCast;
use web_sys::HtmlInputElement;
use yew::prelude::*;

#[hook]
pub fn use_suggest(input_ref: &NodeRef, type_code: TypeCode) -> UseStateHandle<SuggestResults> {
    let lists_ctx = use_lists_context();
    let results = use_state(|| SuggestResults::default());

    {
        log::debug!("use_suggest");
        let lists_ctx = lists_ctx.clone();
        let lists_ctx_2 = lists_ctx.clone();
        let results = results.clone();
        use_oninput(
            input_ref,
            move |e| {
                let lists_ctx = lists_ctx.clone();

                let input_target = e
                    .target()
                    .and_then(|t| t.dyn_into::<HtmlInputElement>().ok());
                input_target.map(|input_target| {
                    let value = input_target.value();

                    log::debug!("value: {}", &value);

                    //TODO?: e.codeによってlists_ctx.tags OR results.map()を使うか分ける。
                    //削除方法はDeleteとBackSpaceの他に、選択からの切り取りがある。その切り取りに対してInput Eventでは対応できない(はず)。よって見送り。

                    let value: &str = &value;
                    let splited_last_value = value
                        .rsplit_once(",")
                        .map_or(value, |tupple| tupple.1.trim_start());
                    // let splited_last_value = value
                    //     .rsplit_once(",")
                    //     .map_or(&value as &str, |tupple| tupple.1.trim_start());

                    let command =
                        SuggestCommand::new(lists_ctx.inner().typed_lists_iter(type_code));
                    let match_command = FuzzyMatchCommand::new(&splited_last_value);

                    let list_infos = command.suggest(match_command);
                    // log::debug!("lists_ctx: {:?}", &lists_ctx);
                    // log::debug!("list_infos: {:?}", &list_infos);
                    results.set(list_infos);
                });
            },
            lists_ctx_2,
        );
    }
    results
}
