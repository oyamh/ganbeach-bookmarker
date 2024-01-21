#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]

mod handler;

use crate::handler::{
    handle_initialize_database, handle_listen_hotkeys, handle_listen_messages,
    handle_listen_toolbar, handle_register_cleaner,
};
use config::{
    DATABASE_VERSION, HISTORY_DATABASE_NAME, HISTORY_PRIMARY_KEY, LISTS_DATABASE_NAME,
    LISTS_INDEX_KEY, LISTS_PRIMARY_KEY, USER_DATA_DATABASE_NAME, USER_DATA_PRIMARY_KEY,
};
use domain::DatabaseConfig;
use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::spawn_local;

#[wasm_bindgen(start)]
pub fn main() -> Result<(), JsValue> {
    wasm_logger::init(wasm_logger::Config::new(log::Level::Trace));
    // wasm_logger::init(wasm_logger::Config::new(log::Level::Debug));
    // wasm_logger::init(wasm_logger::Config::new(log::Level::Error));
    std::panic::set_hook(Box::new(console_error_panic_hook::hook));
    log::debug!("Hello World from Background Script 1");

    handle_listen_hotkeys();

    handle_listen_messages();

    handle_register_cleaner();

    handle_listen_toolbar();

    spawn_local(async {
        log::debug!("BEGIN  span_local");

        //TODO: ConfigBuilderを作って運用する。
        let config = DatabaseConfig::new(DATABASE_VERSION);
        let user_data_config = config
            .new_table_config(USER_DATA_DATABASE_NAME)
            .with_primary_key(USER_DATA_PRIMARY_KEY);
        let lists_config = config
            .new_table_config(LISTS_DATABASE_NAME)
            .with_primary_key(LISTS_PRIMARY_KEY)
            .with_index_keys(vec![LISTS_INDEX_KEY]);
        let history_config = config
            .new_table_config(HISTORY_DATABASE_NAME)
            .with_primary_key(HISTORY_PRIMARY_KEY);
        let config = config
            .with(user_data_config)
            .with(lists_config)
            .with(history_config);

        // refresh token有無に関わらず、初期化だけは行っておく。
        if let Err(error) = handle_initialize_database(config).await {
            log::error!("spawn_local initialize database error: {error:?}");
            return;
        }

        log::debug!("END    span_local");
    });

    // //#[cfg(not(feature = "wasm"))]
    // //{
    // //    //wasmではないコンパイル時の起動方法
    // //    env_logger::init();
    // //    pollster::block_on(run())
    // //}
    // //#[cfg(feature = "wasm")]
    // //{
    // //    //wasmコンパイル時の起動方法
    // //}

    Ok(())
}

// // まだtokio::testはwasmに対応していない。
// // [meta: Stabilize WASI support #4827](https://github.com/tokio-rs/tokio/issues/4827)
// #[cfg(test)]
// #[cfg(target_arch = "wasm32")]
// pub mod test {
//     use crate::*;

//     async fn open_any_db() -> (crate::IdbDatabase, String) {
//         let db_name = uuid::Uuid::new_v4().to_string();
//         let store_name = uuid::Uuid::new_v4().to_string();
//         let mut request =
//             crate::IdbDatabase::open_with_u32(&db_name, 1).expect("IdbDatabase::open_with_u32");
//         let store_name_cloned = store_name.clone();
//         request.set_on_update_needed(move |e: &crate::IdbVersionChangeEvent| {
//             e.db().create_object_store_with_optional_parameters(
//                 &store_name_cloned,
//                 // crate::IdbObjectStoreParameters::new().key_path(Some(&PRIMARY_KEY.into())),
//                 &crate::IdbObjectStoreParameters::new(),
//             )?;
//             Ok(())
//         });
//         (request.await.expect("request.await"), store_name)
//     }

//     //wasm-pack test --firefox --headless --lib -- test::should_return_object_store_names --nocapture
//     //--package background
//     #[wasm_bindgen_test::wasm_bindgen_test]
//     async fn should_return_object_store_names() {
//         let (db, store_name) = open_any_db().await;
//         let tx = db
//             .tx_with_str_and_mode(&store_name, web_sys::IdbTransactionMode::Readwrite)
//             .expect("tx");
//         let store_names: Vec<String> = tx.object_store_names().collect();
//         assert_eq!(store_names, vec![store_name; 1]);
//     }

//     //wasm-pack遅いしprint出力使えないから、web-extで直接デバッグする。
//     //ディレクトリsrc/backgroundに移動してから以下のコマンドを実行する。
//     //wasm-pack test --firefox --headless --lib -- test::should_insert_data --nocapture
//     // #[tokio::test]
//     #[wasm_bindgen_test::wasm_bindgen_test]
//     async fn should_insert_data() {
//         let (db, store_name) = open_any_db().await;
//         log::debug!("db: {db:?}");
//         log::debug!("store_name: {store_name}");
//         // let tx = db
//         //     .tx_with_str_and_mode(&store_name, web_sys::IdbTransactionMode::Readwrite)
//         //     .expect("tx");
//         // let store = tx.object_store(&store_name).expect("object store");

//         // store
//         //     .put_with_key("testkey", &JsValue::from_str("test value"))
//         //     .expect("put with key");
//         // tx.await.expect("await");

//         // //TODO: getで確かめる
//         // let tx = db
//         //     .tx_with_str_and_mode(&store_name, web_sys::IdbTransactionMode::Readwrite)
//         //     .expect("tx");
//         // let store = tx.object_store(DATABASE_NAME).expect("object store");
//         // store.get("testkey").expect("get");
//         // let result = tx.await.expect("await");
//         // println!("{:?}", result);
//     }
// }
